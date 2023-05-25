// #![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::wildcard_imports)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! Rust implementation of [dynamic reconfigure](https://github.com/ros/dynamic_reconfigure).
//!
//! # Missing Features
//!
//! - running in non singleton mode
//! - reporting "level" i.e. what fields were changed
//! - partial updates

use std::borrow::Borrow;
use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use derive_more::{AsRef, Display, From};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use rosrust::api::error::Result;
use rosrust::error::naming::ErrorKind;
use rosrust::error::Error;
use rosrust::{publish, service, Publisher, Service};
use serde::{Deserialize, Serialize};

#[cfg(not(feature="builtin-msgs"))]
mod msg {
    #![allow(clippy::all)]
    rosrust::rosmsg_include!(dynamic_reconfigure/ConfigDescription, dynamic_reconfigure/Reconfigure);
    pub use dynamic_reconfigure::*;
}
#[cfg(feature="builtin-msgs")]
mod msg;

#[derive(Serialize, Deserialize, From, Clone, Debug, Display, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Value {
    #[from(types("&str", "&String"))]
    Str(String),
    #[from(types(u16))]
    Int(i32),
    #[from]
    Float(f64),
    #[from]
    Bool(bool),
}

impl Value {
    pub fn as_string(self, name: &str) -> Result<String> {
        match self {
            Value::Str(value) => Ok(value),
            _ => Err(Error::from_kind(
                ErrorKind::Msg(format!("expected {name} to be a string, was `{self:?}`")).into(),
            )),
        }
    }

    pub fn as_int(&self, name: &str) -> Result<i32> {
        match self {
            Value::Int(value) => Ok(*value),
            _ => Err(Error::from_kind(
                ErrorKind::Msg(format!("expected {name} to be an integer, was `{self:?}`")).into(),
            )),
        }
    }

    pub fn as_float(&self, name: &str) -> Result<f64> {
        match self {
            Value::Float(value) => Ok(*value),
            _ => Err(Error::from_kind(
                ErrorKind::Msg(format!("expected {name} to be a float, was `{self:?}`")).into(),
            )),
        }
    }

    pub fn as_bool(&self, name: &str) -> Result<bool> {
        match self {
            Value::Bool(value) => Ok(*value),
            _ => Err(Error::from_kind(
                ErrorKind::Msg(format!("expected {name} to be a bool, was `{self:?}`")).into(),
            )),
        }
    }

    pub fn type_(&self) -> Type {
        match self {
            Value::Str(_) => Type::String,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
        }
    }
}

pub fn msg_from_iterator(iter: impl IntoIterator<Item = (String, Value)>) -> msg::Config {
    let mut cfg = msg::Config::default();
    for (name, value) in iter {
        match value {
            Value::Str(value) => cfg.strs.push(msg::StrParameter { name, value }),
            Value::Int(value) => cfg.ints.push(msg::IntParameter { name, value }),
            Value::Float(value) => cfg.doubles.push(msg::DoubleParameter { name, value }),
            Value::Bool(value) => cfg.bools.push(msg::BoolParameter { name, value }),
        }
    }
    cfg
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Serialize)]
pub enum Type {
    #[display(fmt = "str")]
    #[serde(rename = "str")]
    String,
    #[display(fmt = "bool")]
    #[serde(rename = "bool")]
    Bool,
    #[display(fmt = "int")]
    #[serde(rename = "int")]
    Int,
    #[display(fmt = "double")]
    #[serde(rename = "double")]
    Float,
}
impl Type {
    fn default(&self) -> Value {
        match self {
            Type::String => String::new().into(),
            Type::Bool => false.into(),
            Type::Int => 0.into(),
            Type::Float => 0.0.into(),
        }
    }

    fn min(&self) -> Value {
        match self {
            Type::String => String::new().into(),
            Type::Bool => false.into(),
            Type::Int => i32::MIN.into(),
            Type::Float => f64::NEG_INFINITY.into(),
        }
    }

    fn max(&self) -> Value {
        match self {
            Type::String => String::new().into(),
            Type::Bool => false.into(),
            Type::Int => i32::MAX.into(),
            Type::Float => f64::INFINITY.into(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct Variant {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub value: Value,
    pub description: String,
}

impl<T: Into<Value>> From<T> for Variant {
    fn from(value: T) -> Self {
        let value = value.into();
        Self {
            name: value.to_string(),
            type_: value.type_(),
            value,
            description: String::new(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct Enum {
    #[serde(rename = "enum_description")]
    pub description: String,
    #[serde(rename = "enum")]
    pub variants: Vec<Variant>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Property {
    pub name: String,
    pub description: String,
    pub type_: Type,
    pub value: Value,
    pub default: Value,
    pub min: Value,
    pub max: Value,
    pub enum_: Option<Enum>,
    pub group: i32,
    pub level: u32,
}

impl Property {
    /// Creates a new Property.
    pub fn new<T: Into<Value>>(name: impl ToString, value: T) -> Self {
        let value = value.into();
        let default = value.type_().default();
        Self::new_default(name, value, default)
    }

    /// Creates a new Property.
    pub fn new_default<T: Into<Value>>(name: impl ToString, value: T, default: T) -> Self {
        let value = value.into();
        let default = default.into();
        let min = value.type_().min();
        let max = value.type_().max();
        Self::new_default_range(name, value, default, min, max)
    }

    /// Creates a new Property.
    pub fn new_range<T: Into<Value>>(name: impl ToString, value: T, min: T, max: T) -> Self {
        let value = value.into();
        let mut default = value.type_().default();
        let min = min.into();
        let max = max.into();
        if default < min {
            default = min.clone();
        } else if default > max {
            default = max.clone();
        }
        Self::new_default_range(name, value, default, min, max)
    }

    /// Creates a new Property.
    pub fn new_default_range<T: Into<Value>>(
        name: impl ToString,
        value: T,
        default: T,
        min: T,
        max: T,
    ) -> Self {
        let value = value.into();
        let default = default.into();
        let min = min.into();
        let max = max.into();
        let type_ = value.type_();

        assert!(
            type_ == default.type_() && type_ == min.type_() && value.type_() == max.type_(),
            "value ({}), default ({}), min ({}) and max ({}) should be the same type",
            type_,
            default.type_(),
            min.type_(),
            max.type_()
        );

        assert!(
            min <= max,
            "min ({min:?}) should be less than default ({default:?}) should be less than max \
             ({max:?})"
        );

        Self {
            name: name.to_string(),
            description: String::new(),
            type_,
            value,
            default,
            min,
            max,
            enum_: None,
            group: 0,
            level: 0,
        }
    }

    /// Creates a new Enum Property.
    ///
    /// First variant is default.
    pub fn new_enum<T: Into<Value>, V: Into<Variant>>(
        name: impl ToString,
        value: T,
        variants: impl IntoIterator<Item = V>,
    ) -> Self {
        // TODO merge with next
        let variants: Vec<_> = variants.into_iter().map(Into::into).collect();

        assert!(!variants.is_empty(), "enum variants cannot be empty");

        let value = value.into();

        let default = &variants[0].value;
        let mut min = default;
        let mut max = default;

        let mut value_valid = default == &value;

        for Variant { value: v, .. } in &variants[1..] {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
            value_valid |= v == &value;
        }

        assert!(value_valid, "value not in variants");

        let mut this =
            Self::new_default_range(name, value, default.clone(), min.clone(), max.clone());
        this.enum_ = Some(Enum {
            description: String::new(),
            variants,
        });
        this
    }

    /// Creates a new Enum Property.
    pub fn new_enum_default<T: Into<Value>, V: Into<Variant>>(
        name: impl ToString,
        value: T,
        default: T,
        variants: impl IntoIterator<Item = V>,
    ) -> Self {
        let variants: Vec<_> = variants.into_iter().map(Into::into).collect();

        assert!(!variants.is_empty(), "enum variants cannot be empty");

        let value = value.into();
        let default = default.into();
        let mut min = &default;
        let mut max = &default;

        let mut value_valid = false;
        let mut default_valid = false;

        for Variant { value: v, .. } in &variants[0..] {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
            value_valid |= v == &value;
            default_valid |= v == &default;
        }

        assert!(value_valid, "value not in variants");
        assert!(default_valid, "default not in variants");

        let mut this =
            Self::new_default_range(name, value, default.clone(), min.clone(), max.clone());
        this.enum_ = Some(Enum {
            description: String::new(),
            variants,
        });
        this
    }

    pub fn group(mut self, id: i32) -> Self {
        self.group = id;
        self
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = description.to_string();
        self
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Display)]
pub enum GroupType {
    #[display(fmt = "")]
    Normal,
    #[display(fmt = "collapse")]
    Collapse,
    #[display(fmt = "tab")]
    Tab,
    #[display(fmt = "hide")]
    Hide,
    #[display(fmt = "apply")]
    Apply,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Group {
    pub name: String,
    pub state: bool,
    pub id: i32,
    pub parent: i32,
    pub type_: GroupType,
}

impl From<Group> for msg::GroupState {
    fn from(
        Group {
            name,
            state,
            id,
            parent,
            ..
        }: Group,
    ) -> Self {
        msg::GroupState {
            name,
            state,
            id,
            parent,
        }
    }
}

#[derive(Clone, Debug, AsRef)]
struct PropertyByName(Property);
impl Eq for PropertyByName {}
impl PartialEq for PropertyByName {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}
impl Hash for PropertyByName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}
impl Borrow<str> for PropertyByName {
    fn borrow(&self) -> &str {
        &self.0.name
    }
}

#[derive(Clone, Debug, AsRef)]
struct GroupById(Group);
impl PartialEq for GroupById {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DynConfig {
    properties: HashSet<PropertyByName>,
    groups: BTreeSet<GroupById>,
}

fn insert_property(config: &mut msg::Config, name: String, value: Value) {
    match value {
        Value::Str(value) => config.strs.push(msg::StrParameter { name, value }),
        Value::Int(value) => config.ints.push(msg::IntParameter { name, value }),
        Value::Float(value) => config.doubles.push(msg::DoubleParameter { name, value }),
        Value::Bool(value) => config.bools.push(msg::BoolParameter { name, value }),
    }
}

impl Config for DynConfig {
    fn groups(&self) -> Vec<Group> {
        self.groups.iter().map(AsRef::as_ref).cloned().collect()
    }

    fn properties(&self) -> Vec<Property> {
        self.properties.iter().map(AsRef::as_ref).cloned().collect()
    }

    fn set(&mut self, name: &str, value: Value) -> Result<()> {
        let prop = &self
            .properties
            .get(name)
            .ok_or_else(|| format!("{name} is not a valid property"))?
            .0;
        if prop.type_ != value.type_() {
            return Err(format!(
                "expected {} for {name}, but was {}",
                prop.type_,
                value.type_()
            )
            .into());
        }
        let mut prop = self.properties.take(name).unwrap().0;
        prop.value = value;
        self.properties.insert(PropertyByName(prop));
        Ok(())
    }
}

fn set_property(namespace: &str, name: &str, value: &Value) -> std::result::Result<(), String> {
    rosrust::param(&format!("{namespace}{name}"))
        .ok_or_else(|| format!("unable to get propety `{namespace}{name}`"))?
        .set(value)
        .map_err(|e| format!("unable to set propety `{namespace}{name}`: {e}"))
}

pub trait Config: Sync + Send + Clone + 'static {
    fn clean_up(&mut self) {}
    fn apply_update(
        &mut self,
        msg::Config {
            bools,
            ints,
            strs,
            doubles,
            ..
        }: msg::Config,
    ) -> Result<()> {
        for (name, value) in bools
            .into_iter()
            .map(|msg::BoolParameter { name, value }| (name, Value::from(value)))
            .chain(
                ints.into_iter()
                    .map(|msg::IntParameter { name, value }| (name, value.into())),
            )
            .chain(
                strs.into_iter()
                    .map(|msg::StrParameter { name, value }| (name, value.into())),
            )
            .chain(
                doubles
                    .into_iter()
                    .map(|msg::DoubleParameter { name, value }| (name, value.into())),
            )
        {
            eprintln!("{name} = {value}");
            self.set(&name, value)?;
        }
        self.clean_up();
        // TODO Apply group updates
        Ok(())
    }
    fn groups(&self) -> Vec<Group>;
    fn description_msg(&self) -> msg::ConfigDescription {
        let mut max = msg::Config::default();
        let mut min = msg::Config::default();
        let mut dflt = msg::Config::default();
        let mut groups = Vec::new();
        for Group {
            id,
            name,
            parent,
            type_,
            ..
        } in self.groups().into_iter()
        {
            let mut group = msg::Group {
                name: name.clone(),
                type_: type_.to_string(),
                parameters: Default::default(),
                parent,
                id,
            };
            for Property {
                name,
                description,
                type_,
                default,
                enum_,
                level,
                max: mx,
                min: mn,
                ..
            } in self.properties().into_iter().filter(|p| p.group == id)
            {
                insert_property(&mut max, name.clone(), mx.clone());
                insert_property(&mut min, name.clone(), mn.clone());
                insert_property(&mut dflt, name.clone(), default.clone());
                group.parameters.push(msg::ParamDescription {
                    name: name.clone(),
                    type_: type_.to_string(),
                    level,
                    description: description.clone(),
                    edit_method: enum_
                        .as_ref()
                        .map(serde_json::to_string)
                        .transpose()
                        .expect("unable to serialize enum_")
                        .unwrap_or_default(),
                })
            }
            groups.push(group)
        }
        msg::ConfigDescription {
            groups,
            max,
            min,
            dflt,
        }
    }
    fn config_msg(&self) -> msg::Config {
        let mut config = msg::Config {
            groups: self.groups().into_iter().map(From::from).collect(),
            ..Default::default()
        };
        for Property { name, value, .. } in self.properties() {
            insert_property(&mut config, name, value)
        }

        config
    }
    fn properties(&self) -> Vec<Property>;
    fn keys(&self) -> Vec<String> {
        self.properties().into_iter().map(|p| p.name).collect()
    }
    fn set(&mut self, name: &str, value: Value) -> Result<()>;
}

#[derive(Clone)]
pub struct Updating<C>(Arc<RwLock<C>>);
impl<C> Updating<C> {
    /// Gets read access to the config.
    ///
    /// Make sure to drop the returned guard to avoid deadlocks, or use
    /// [`copy()`](Self::copy).
    pub fn read(&self) -> RwLockReadGuard<C> {
        self.0.try_read_for(Duration::from_secs(1)).expect("DEADLOCK")
    }

    /// Gets a copy of the config.
    ///
    /// Returns a copy of the underlying config.
    pub fn copy(&self) -> C
    where
        C: Clone,
    {
        self.0.try_read_for(Duration::from_secs(1)).expect("DEADLOCK").deref().clone()
    }
}

pub struct Server<C: Config> {
    config: Arc<RwLock<C>>,
    namespace: String,
    update_topic: Publisher<msg::Config>,
    descr_topic: Publisher<msg::ConfigDescription>,
    _service: Service,
    description_msg: Arc<Mutex<msg::ConfigDescription>>,
    config_msg: Arc<Mutex<msg::Config>>,
}

impl<C: Config> Server<C> {
    pub fn empty() -> Result<Self>
    where
        C: Default,
    {
        Self::empty_with_namespace("~")
    }

    pub fn empty_with_namespace(namespace: &str) -> Result<Self>
    where
        C: Default,
    {
        Self::with_namespace(namespace, Default::default())
    }

    pub fn new(config: C) -> Result<Self> {
        Self::with_namespace("~", config)
    }

    pub fn with_namespace(namespace: &str, mut config: C) -> Result<Server<C>> {
        let namespace = format!(
            "{}{namespace}{}",
            (!namespace.starts_with(['~', '/']))
                .then_some("~")
                .unwrap_or_default(),
            (!(namespace.is_empty() || namespace == "~" || namespace.ends_with('/')))
                .then_some("/")
                .unwrap_or_default(),
        );

        let mut update_topic = publish(&format!("{namespace}parameter_updates"), 10)?;
        update_topic.set_latching(true);
        let mut descr_topic = publish(&format!("{namespace}parameter_descriptions"), 10)?;
        descr_topic.set_latching(true);

        for name in config.keys() {
            let param = rosrust::param(&format!("{namespace}{name}"))
                .ok_or_else(|| format!("unable to get param: `{namespace}{name}`"))?;
            if param.exists()? {
                config.set(name.as_str(), param.get()?)?;
            }
            config.clean_up();
        }

        update_topic.send(config.config_msg())?;
        descr_topic.send(config.description_msg())?;

        let config_msg = Arc::new(Mutex::new(config.config_msg()));
        let description_msg = Arc::new(Mutex::new(config.description_msg()));
        let config = Arc::new(RwLock::new(config));

        let service = {
            let config = config.clone();
            let config_msg = config_msg.clone();
            let description_msg = description_msg.clone();
            let update_topic = update_topic.clone();
            let description_topic = descr_topic.clone();
            let namespace = namespace.clone();
            service::<msg::Reconfigure, _>(&format!("{}set_parameters", namespace), move |req| {
                // TODO figure out what to do with poisned mutexes
                // TODO maybe optimize mutex durations
                // TODO error on long mutex log
                let mut config = config.try_write_for(Duration::from_secs(1)).expect("DEADLOCK");
                config
                    .apply_update(req.config)
                    .map_err(|e| format!("error applying config: {e}"))?;
                let updated_config_msg = config.config_msg();
                {
                    let mut config_msg = config_msg.try_lock_for(Duration::from_secs(1)).expect("DEADLOCK");
                    if *config_msg != updated_config_msg {
                        *config_msg = updated_config_msg.clone();
                        update_topic
                            .send(updated_config_msg.clone())
                            .map_err(|e| format!("sending update: {e}"))?;
                    }
                }
                {
                    let updated_description_msg = config.description_msg();
                    let mut description_msg =
                        description_msg.try_lock_for(Duration::from_secs(1)).expect("DEADLOCK");
                    if *description_msg != updated_description_msg {
                        *description_msg = updated_description_msg.clone();
                        description_topic
                            .send(updated_description_msg)
                            .map_err(|e| format!("sending update: {e}"))?;
                    }
                }
                for Property { name, value, .. } in config.properties() {
                    set_property(&namespace, &name, &value)?;
                }
                Ok(msg::ReconfigureRes {
                    config: updated_config_msg,
                })
            })?
        };

        Ok(Self {
            config,
            update_topic,
            descr_topic,
            _service: service,
            namespace,
            description_msg,
            config_msg,
        })
    }

    /// Gets an owned value of the config.
    ///
    /// Configuration will not update.
    pub fn get_config(&self) -> C {
        self.config.try_read_for(Duration::from_secs(1)).expect("DEADLOCK").clone()
    }

    /// Gets an updating reference to the config.
    pub fn get_config_updating(&mut self) -> Updating<C> {
        Updating(self.config.clone())
    }

    /// Set the config and send update message.
    ///
    /// **Note:** this will overwrite any changes recieved via update messages.
    /// It is good practise to call [`get_config()`](Self::get_config)
    /// first.
    pub fn set_config(&mut self, config: C) -> Result<()> {
        let config_msg = config.config_msg();
        let descr_msg = config.description_msg();
        let properties = config.properties();

        *self.config.try_write_for(Duration::from_secs(1)).expect("DEADLOCK") = config;

        if config_msg != *self.config_msg.try_lock_for(Duration::from_secs(1)).expect("DEADLOCK") {
            self.update_topic.send(config_msg)?;
        }

        if descr_msg != *self.description_msg.try_lock_for(Duration::from_secs(1)).expect("DEADLOCK") {
            self.descr_topic.send(descr_msg)?;
        }

        for Property { name, value, .. } in properties {
            set_property(&self.namespace, &name, &value)?;
        }

        Ok(())
    }
}
