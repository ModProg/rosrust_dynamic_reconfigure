#![allow(unused, clippy::all)]
pub mod dynamic_reconfigure {
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct BoolParameter {
        pub name: ::std::string::String,
        pub value: bool,
    }
    impl BoolParameter {}

    impl std::convert::From<BoolParameter> for rosrust::MsgValue {
        fn from(src: BoolParameter) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<BoolParameter> for rosrust::MsgMessage {
        fn from(src: BoolParameter) -> Self {
            let mut output = Self::new();
            output.insert("name".into(), src.name.into());
            output.insert("value".into(), src.value.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for BoolParameter {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for BoolParameter {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                name: src.remove("name").ok_or(())?.try_into()?,
                value: src.remove("value").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for BoolParameter {
        fn eq(&self, other: &Self) -> bool {
            true && self.name == other.name && self.value == other.value
        }
    }
    impl std::fmt::Debug for BoolParameter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(BoolParameter))
                .field(stringify!(name), &self.name)
                .field(stringify!(value), &self.value)
                .finish()
        }
    }
    impl Default for BoolParameter {
        fn default() -> Self {
            Self {
                name: Default::default(),
                value: Default::default(),
            }
        }
    }
    impl rosrust::Message for BoolParameter {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "string name\nbool value\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "23f05028c1a699fb83e22401228c3a9e".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/BoolParameter".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for BoolParameter {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.name.encode(w.by_ref())?;
            self.value.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                name: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                value: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ParamDescription {
        pub name: ::std::string::String,
        pub type_: ::std::string::String,
        pub level: u32,
        pub description: ::std::string::String,
        pub edit_method: ::std::string::String,
    }
    impl ParamDescription {}

    impl std::convert::From<ParamDescription> for rosrust::MsgValue {
        fn from(src: ParamDescription) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<ParamDescription> for rosrust::MsgMessage {
        fn from(src: ParamDescription) -> Self {
            let mut output = Self::new();
            output.insert("name".into(), src.name.into());
            output.insert("type".into(), src.type_.into());
            output.insert("level".into(), src.level.into());
            output.insert("description".into(), src.description.into());
            output.insert("edit_method".into(), src.edit_method.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for ParamDescription {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for ParamDescription {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                name: src.remove("name").ok_or(())?.try_into()?,
                type_: src.remove("type").ok_or(())?.try_into()?,
                level: src.remove("level").ok_or(())?.try_into()?,
                description: src.remove("description").ok_or(())?.try_into()?,
                edit_method: src.remove("edit_method").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for ParamDescription {
        fn eq(&self, other: &Self) -> bool {
            true && self.name == other.name
                && self.type_ == other.type_
                && self.level == other.level
                && self.description == other.description
                && self.edit_method == other.edit_method
        }
    }
    impl std::fmt::Debug for ParamDescription {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(ParamDescription))
                .field(stringify!(name), &self.name)
                .field(stringify!(type_), &self.type_)
                .field(stringify!(level), &self.level)
                .field(stringify!(description), &self.description)
                .field(stringify!(edit_method), &self.edit_method)
                .finish()
        }
    }
    impl Default for ParamDescription {
        fn default() -> Self {
            Self {
                name: Default::default(),
                type_: Default::default(),
                level: Default::default(),
                description: Default::default(),
                edit_method: Default::default(),
            }
        }
    }
    impl rosrust::Message for ParamDescription {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "string name\nstring type\nuint32 level\nstring description\nstring edit_method\n"
                .into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "7434fcb9348c13054e0c3b267c8cb34d".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/ParamDescription".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for ParamDescription {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.name.encode(w.by_ref())?;
            self.type_.encode(w.by_ref())?;
            self.level.encode(w.by_ref())?;
            self.description.encode(w.by_ref())?;
            self.edit_method.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                name: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                type_: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                level: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                description: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                edit_method: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Config {
        pub bools: Vec<BoolParameter>,
        pub ints: Vec<IntParameter>,
        pub strs: Vec<StrParameter>,
        pub doubles: Vec<DoubleParameter>,
        pub groups: Vec<GroupState>,
    }
    impl Config {}

    impl std::convert::From<Config> for rosrust::MsgValue {
        fn from(src: Config) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Config> for rosrust::MsgMessage {
        fn from(src: Config) -> Self {
            let mut output = Self::new();
            output.insert("bools".into(), src.bools.into());
            output.insert("ints".into(), src.ints.into());
            output.insert("strs".into(), src.strs.into());
            output.insert("doubles".into(), src.doubles.into());
            output.insert("groups".into(), src.groups.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Config {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Config {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                bools: src.remove("bools").ok_or(())?.try_into()?,
                ints: src.remove("ints").ok_or(())?.try_into()?,
                strs: src.remove("strs").ok_or(())?.try_into()?,
                doubles: src.remove("doubles").ok_or(())?.try_into()?,
                groups: src.remove("groups").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Config {
        fn eq(&self, other: &Self) -> bool {
            true && self.bools == other.bools
                && self.ints == other.ints
                && self.strs == other.strs
                && self.doubles == other.doubles
                && self.groups == other.groups
        }
    }
    impl std::fmt::Debug for Config {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Config))
                .field(stringify!(bools), &self.bools)
                .field(stringify!(ints), &self.ints)
                .field(stringify!(strs), &self.strs)
                .field(stringify!(doubles), &self.doubles)
                .field(stringify!(groups), &self.groups)
                .finish()
        }
    }
    impl Default for Config {
        fn default() -> Self {
            Self {
                bools: Default::default(),
                ints: Default::default(),
                strs: Default::default(),
                doubles: Default::default(),
                groups: Default::default(),
            }
        }
    }
    impl rosrust::Message for Config {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "BoolParameter[] bools\nIntParameter[] ints\nStrParameter[] strs\nDoubleParameter[] doubles\nGroupState[] groups\n\n================================================================================\nMSG: dynamic_reconfigure/BoolParameter\nstring name\nbool value\n\n================================================================================\nMSG: dynamic_reconfigure/IntParameter\nstring name\nint32 value\n\n================================================================================\nMSG: dynamic_reconfigure/StrParameter\nstring name\nstring value\n\n================================================================================\nMSG: dynamic_reconfigure/DoubleParameter\nstring name\nfloat64 value\n\n================================================================================\nMSG: dynamic_reconfigure/GroupState\nstring name\nbool state\nint32 id\nint32 parent\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "958f16a05573709014982821e6822580".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/Config".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Config {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            rosrust::rosmsg::encode_variable_slice(&self.bools, w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.ints, w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.strs, w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.doubles, w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.groups, w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                bools: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                ints: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                strs: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                doubles: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                groups: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct GroupState {
        pub name: ::std::string::String,
        pub state: bool,
        pub id: i32,
        pub parent: i32,
    }
    impl GroupState {}

    impl std::convert::From<GroupState> for rosrust::MsgValue {
        fn from(src: GroupState) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<GroupState> for rosrust::MsgMessage {
        fn from(src: GroupState) -> Self {
            let mut output = Self::new();
            output.insert("name".into(), src.name.into());
            output.insert("state".into(), src.state.into());
            output.insert("id".into(), src.id.into());
            output.insert("parent".into(), src.parent.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for GroupState {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for GroupState {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                name: src.remove("name").ok_or(())?.try_into()?,
                state: src.remove("state").ok_or(())?.try_into()?,
                id: src.remove("id").ok_or(())?.try_into()?,
                parent: src.remove("parent").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for GroupState {
        fn eq(&self, other: &Self) -> bool {
            true && self.name == other.name
                && self.state == other.state
                && self.id == other.id
                && self.parent == other.parent
        }
    }
    impl std::fmt::Debug for GroupState {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(GroupState))
                .field(stringify!(name), &self.name)
                .field(stringify!(state), &self.state)
                .field(stringify!(id), &self.id)
                .field(stringify!(parent), &self.parent)
                .finish()
        }
    }
    impl Default for GroupState {
        fn default() -> Self {
            Self {
                name: Default::default(),
                state: Default::default(),
                id: Default::default(),
                parent: Default::default(),
            }
        }
    }
    impl rosrust::Message for GroupState {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "string name\nbool state\nint32 id\nint32 parent\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "a2d87f51dc22930325041a2f8b1571f8".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/GroupState".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for GroupState {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.name.encode(w.by_ref())?;
            self.state.encode(w.by_ref())?;
            self.id.encode(w.by_ref())?;
            self.parent.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                name: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                state: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                id: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                parent: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct StrParameter {
        pub name: ::std::string::String,
        pub value: ::std::string::String,
    }
    impl StrParameter {}

    impl std::convert::From<StrParameter> for rosrust::MsgValue {
        fn from(src: StrParameter) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<StrParameter> for rosrust::MsgMessage {
        fn from(src: StrParameter) -> Self {
            let mut output = Self::new();
            output.insert("name".into(), src.name.into());
            output.insert("value".into(), src.value.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for StrParameter {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for StrParameter {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                name: src.remove("name").ok_or(())?.try_into()?,
                value: src.remove("value").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for StrParameter {
        fn eq(&self, other: &Self) -> bool {
            true && self.name == other.name && self.value == other.value
        }
    }
    impl std::fmt::Debug for StrParameter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(StrParameter))
                .field(stringify!(name), &self.name)
                .field(stringify!(value), &self.value)
                .finish()
        }
    }
    impl Default for StrParameter {
        fn default() -> Self {
            Self {
                name: Default::default(),
                value: Default::default(),
            }
        }
    }
    impl rosrust::Message for StrParameter {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "string name\nstring value\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "bc6ccc4a57f61779c8eaae61e9f422e0".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/StrParameter".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for StrParameter {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.name.encode(w.by_ref())?;
            self.value.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                name: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                value: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ReconfigureReq {
        pub config: Config,
    }
    impl ReconfigureReq {}

    impl std::convert::From<ReconfigureReq> for rosrust::MsgValue {
        fn from(src: ReconfigureReq) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<ReconfigureReq> for rosrust::MsgMessage {
        fn from(src: ReconfigureReq) -> Self {
            let mut output = Self::new();
            output.insert("config".into(), src.config.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for ReconfigureReq {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for ReconfigureReq {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                config: src.remove("config").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for ReconfigureReq {
        fn eq(&self, other: &Self) -> bool {
            true && self.config == other.config
        }
    }
    impl std::fmt::Debug for ReconfigureReq {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(ReconfigureReq))
                .field(stringify!(config), &self.config)
                .finish()
        }
    }
    impl Default for ReconfigureReq {
        fn default() -> Self {
            Self {
                config: Default::default(),
            }
        }
    }
    impl rosrust::Message for ReconfigureReq {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "Config config\n\n================================================================================\nMSG: dynamic_reconfigure/Config\nBoolParameter[] bools\nIntParameter[] ints\nStrParameter[] strs\nDoubleParameter[] doubles\nGroupState[] groups\n\n================================================================================\nMSG: dynamic_reconfigure/BoolParameter\nstring name\nbool value\n\n================================================================================\nMSG: dynamic_reconfigure/IntParameter\nstring name\nint32 value\n\n================================================================================\nMSG: dynamic_reconfigure/StrParameter\nstring name\nstring value\n\n================================================================================\nMSG: dynamic_reconfigure/DoubleParameter\nstring name\nfloat64 value\n\n================================================================================\nMSG: dynamic_reconfigure/GroupState\nstring name\nbool state\nint32 id\nint32 parent\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "ac41a77620a4a0348b7001641796a8a1".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/ReconfigureReq".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for ReconfigureReq {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.config.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                config: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ReconfigureRes {
        pub config: Config,
    }
    impl ReconfigureRes {}

    impl std::convert::From<ReconfigureRes> for rosrust::MsgValue {
        fn from(src: ReconfigureRes) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<ReconfigureRes> for rosrust::MsgMessage {
        fn from(src: ReconfigureRes) -> Self {
            let mut output = Self::new();
            output.insert("config".into(), src.config.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for ReconfigureRes {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for ReconfigureRes {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                config: src.remove("config").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for ReconfigureRes {
        fn eq(&self, other: &Self) -> bool {
            true && self.config == other.config
        }
    }
    impl std::fmt::Debug for ReconfigureRes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(ReconfigureRes))
                .field(stringify!(config), &self.config)
                .finish()
        }
    }
    impl Default for ReconfigureRes {
        fn default() -> Self {
            Self {
                config: Default::default(),
            }
        }
    }
    impl rosrust::Message for ReconfigureRes {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "Config config\n\n================================================================================\nMSG: dynamic_reconfigure/Config\nBoolParameter[] bools\nIntParameter[] ints\nStrParameter[] strs\nDoubleParameter[] doubles\nGroupState[] groups\n\n================================================================================\nMSG: dynamic_reconfigure/BoolParameter\nstring name\nbool value\n\n================================================================================\nMSG: dynamic_reconfigure/IntParameter\nstring name\nint32 value\n\n================================================================================\nMSG: dynamic_reconfigure/StrParameter\nstring name\nstring value\n\n================================================================================\nMSG: dynamic_reconfigure/DoubleParameter\nstring name\nfloat64 value\n\n================================================================================\nMSG: dynamic_reconfigure/GroupState\nstring name\nbool state\nint32 id\nint32 parent\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "ac41a77620a4a0348b7001641796a8a1".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/ReconfigureRes".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for ReconfigureRes {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.config.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                config: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct DoubleParameter {
        pub name: ::std::string::String,
        pub value: f64,
    }
    impl DoubleParameter {}

    impl std::convert::From<DoubleParameter> for rosrust::MsgValue {
        fn from(src: DoubleParameter) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<DoubleParameter> for rosrust::MsgMessage {
        fn from(src: DoubleParameter) -> Self {
            let mut output = Self::new();
            output.insert("name".into(), src.name.into());
            output.insert("value".into(), src.value.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for DoubleParameter {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for DoubleParameter {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                name: src.remove("name").ok_or(())?.try_into()?,
                value: src.remove("value").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for DoubleParameter {
        fn eq(&self, other: &Self) -> bool {
            true && self.name == other.name && self.value == other.value
        }
    }
    impl std::fmt::Debug for DoubleParameter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(DoubleParameter))
                .field(stringify!(name), &self.name)
                .field(stringify!(value), &self.value)
                .finish()
        }
    }
    impl Default for DoubleParameter {
        fn default() -> Self {
            Self {
                name: Default::default(),
                value: Default::default(),
            }
        }
    }
    impl rosrust::Message for DoubleParameter {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "string name\nfloat64 value\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "d8512f27253c0f65f928a67c329cd658".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/DoubleParameter".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for DoubleParameter {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.name.encode(w.by_ref())?;
            self.value.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                name: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                value: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct IntParameter {
        pub name: ::std::string::String,
        pub value: i32,
    }
    impl IntParameter {}

    impl std::convert::From<IntParameter> for rosrust::MsgValue {
        fn from(src: IntParameter) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<IntParameter> for rosrust::MsgMessage {
        fn from(src: IntParameter) -> Self {
            let mut output = Self::new();
            output.insert("name".into(), src.name.into());
            output.insert("value".into(), src.value.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for IntParameter {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for IntParameter {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                name: src.remove("name").ok_or(())?.try_into()?,
                value: src.remove("value").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for IntParameter {
        fn eq(&self, other: &Self) -> bool {
            true && self.name == other.name && self.value == other.value
        }
    }
    impl std::fmt::Debug for IntParameter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(IntParameter))
                .field(stringify!(name), &self.name)
                .field(stringify!(value), &self.value)
                .finish()
        }
    }
    impl Default for IntParameter {
        fn default() -> Self {
            Self {
                name: Default::default(),
                value: Default::default(),
            }
        }
    }
    impl rosrust::Message for IntParameter {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "string name\nint32 value\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "65fedc7a0cbfb8db035e46194a350bf1".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/IntParameter".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for IntParameter {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.name.encode(w.by_ref())?;
            self.value.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                name: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                value: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ConfigDescription {
        pub groups: Vec<Group>,
        pub max: Config,
        pub min: Config,
        pub dflt: Config,
    }
    impl ConfigDescription {}

    impl std::convert::From<ConfigDescription> for rosrust::MsgValue {
        fn from(src: ConfigDescription) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<ConfigDescription> for rosrust::MsgMessage {
        fn from(src: ConfigDescription) -> Self {
            let mut output = Self::new();
            output.insert("groups".into(), src.groups.into());
            output.insert("max".into(), src.max.into());
            output.insert("min".into(), src.min.into());
            output.insert("dflt".into(), src.dflt.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for ConfigDescription {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for ConfigDescription {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                groups: src.remove("groups").ok_or(())?.try_into()?,
                max: src.remove("max").ok_or(())?.try_into()?,
                min: src.remove("min").ok_or(())?.try_into()?,
                dflt: src.remove("dflt").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for ConfigDescription {
        fn eq(&self, other: &Self) -> bool {
            true && self.groups == other.groups
                && self.max == other.max
                && self.min == other.min
                && self.dflt == other.dflt
        }
    }
    impl std::fmt::Debug for ConfigDescription {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(ConfigDescription))
                .field(stringify!(groups), &self.groups)
                .field(stringify!(max), &self.max)
                .field(stringify!(min), &self.min)
                .field(stringify!(dflt), &self.dflt)
                .finish()
        }
    }
    impl Default for ConfigDescription {
        fn default() -> Self {
            Self {
                groups: Default::default(),
                max: Default::default(),
                min: Default::default(),
                dflt: Default::default(),
            }
        }
    }
    impl rosrust::Message for ConfigDescription {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "Group[] groups\nConfig max\nConfig min\nConfig dflt\n\n================================================================================\nMSG: dynamic_reconfigure/Group\nstring name\nstring type\nParamDescription[] parameters\nint32 parent \nint32 id\n\n================================================================================\nMSG: dynamic_reconfigure/Config\nBoolParameter[] bools\nIntParameter[] ints\nStrParameter[] strs\nDoubleParameter[] doubles\nGroupState[] groups\n\n================================================================================\nMSG: dynamic_reconfigure/ParamDescription\nstring name\nstring type\nuint32 level\nstring description\nstring edit_method\n\n================================================================================\nMSG: dynamic_reconfigure/BoolParameter\nstring name\nbool value\n\n================================================================================\nMSG: dynamic_reconfigure/IntParameter\nstring name\nint32 value\n\n================================================================================\nMSG: dynamic_reconfigure/StrParameter\nstring name\nstring value\n\n================================================================================\nMSG: dynamic_reconfigure/DoubleParameter\nstring name\nfloat64 value\n\n================================================================================\nMSG: dynamic_reconfigure/GroupState\nstring name\nbool state\nint32 id\nint32 parent\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "757ce9d44ba8ddd801bb30bc456f946f".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/ConfigDescription".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for ConfigDescription {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            rosrust::rosmsg::encode_variable_slice(&self.groups, w.by_ref())?;
            self.max.encode(w.by_ref())?;
            self.min.encode(w.by_ref())?;
            self.dflt.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                groups: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                max: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                min: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                dflt: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Group {
        pub name: ::std::string::String,
        pub type_: ::std::string::String,
        pub parameters: Vec<ParamDescription>,
        pub parent: i32,
        pub id: i32,
    }
    impl Group {}

    impl std::convert::From<Group> for rosrust::MsgValue {
        fn from(src: Group) -> Self {
            rosrust::MsgValue::Message(src.into())
        }
    }
    impl std::convert::From<Group> for rosrust::MsgMessage {
        fn from(src: Group) -> Self {
            let mut output = Self::new();
            output.insert("name".into(), src.name.into());
            output.insert("type".into(), src.type_.into());
            output.insert("parameters".into(), src.parameters.into());
            output.insert("parent".into(), src.parent.into());
            output.insert("id".into(), src.id.into());
            output
        }
    }
    impl std::convert::TryFrom<rosrust::MsgValue> for Group {
        type Error = ();

        fn try_from(src: rosrust::MsgValue) -> Result<Self, ()> {
            use std::convert::TryInto;
            let message: rosrust::MsgMessage = src.try_into()?;
            message.try_into()
        }
    }
    impl std::convert::TryFrom<rosrust::MsgMessage> for Group {
        type Error = ();

        fn try_from(mut src: rosrust::MsgMessage) -> Result<Self, ()> {
            use std::convert::TryInto;
            Ok(Self {
                name: src.remove("name").ok_or(())?.try_into()?,
                type_: src.remove("type").ok_or(())?.try_into()?,
                parameters: src.remove("parameters").ok_or(())?.try_into()?,
                parent: src.remove("parent").ok_or(())?.try_into()?,
                id: src.remove("id").ok_or(())?.try_into()?,
            })
        }
    }
    impl std::cmp::PartialEq<Self> for Group {
        fn eq(&self, other: &Self) -> bool {
            true && self.name == other.name
                && self.type_ == other.type_
                && self.parameters == other.parameters
                && self.parent == other.parent
                && self.id == other.id
        }
    }
    impl std::fmt::Debug for Group {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct(stringify!(Group))
                .field(stringify!(name), &self.name)
                .field(stringify!(type_), &self.type_)
                .field(stringify!(parameters), &self.parameters)
                .field(stringify!(parent), &self.parent)
                .field(stringify!(id), &self.id)
                .finish()
        }
    }
    impl Default for Group {
        fn default() -> Self {
            Self {
                name: Default::default(),
                type_: Default::default(),
                parameters: Default::default(),
                parent: Default::default(),
                id: Default::default(),
            }
        }
    }
    impl rosrust::Message for Group {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            "string name\nstring type\nParamDescription[] parameters\nint32 parent \nint32 id\n\n================================================================================\nMSG: dynamic_reconfigure/ParamDescription\nstring name\nstring type\nuint32 level\nstring description\nstring edit_method\n".into()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "9e8cd9e9423c94823db3614dd8b1cf7a".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/Group".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Group {
        fn encode<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
            self.name.encode(w.by_ref())?;
            self.type_.encode(w.by_ref())?;
            rosrust::rosmsg::encode_variable_slice(&self.parameters, w.by_ref())?;
            self.parent.encode(w.by_ref())?;
            self.id.encode(w.by_ref())?;
            Ok(())
        }

        fn decode<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
            Ok(Self {
                name: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                type_: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                parameters: rosrust::rosmsg::decode_variable_vec(r.by_ref())?,
                parent: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
                id: rosrust::rosmsg::RosMsg::decode(r.by_ref())?,
            })
        }
    }
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Reconfigure;

    impl rosrust::Message for Reconfigure {
        #[inline]
        fn msg_definition() -> ::std::string::String {
            String::new()
        }

        #[inline]
        fn md5sum() -> ::std::string::String {
            "bb125d226a21982a4a98760418dc2672".into()
        }

        #[inline]
        fn msg_type() -> ::std::string::String {
            "dynamic_reconfigure/Reconfigure".into()
        }
    }
    impl rosrust::rosmsg::RosMsg for Reconfigure {
        fn encode<W: ::std::io::Write>(&self, _w: W) -> ::std::io::Result<()> {
            Ok(())
        }

        fn decode<R: ::std::io::Read>(_r: R) -> ::std::io::Result<Self> {
            Ok(Self {})
        }
    }
    impl rosrust::ServicePair for Reconfigure {
        type Request = ReconfigureReq;
        type Response = ReconfigureRes;
    }
}
pub use dynamic_reconfigure::*;
