// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Floating IP support.

use std::net;
use std::rc::Rc;

use chrono::{DateTime, FixedOffset};

use super::super::Result;
use super::super::session::Session;
use super::base::V2API;
use super::protocol;


/// Structure representing a single floating IP.
#[derive(Clone, Debug)]
pub struct FloatingIp {
    session: Rc<Session>,
    inner: protocol::FloatingIp
}

impl FloatingIp {
    /// Load a FloatingIp object.
    pub(crate) fn load<Id: AsRef<str>>(session: Rc<Session>, id: Id)
            -> Result<FloatingIp> {
        let inner = session.get_floating_ip(id)?;
        Ok(FloatingIp {
            session: session,
            inner: inner
        })
    }

    transparent_property! {
        #[doc = "Creation data and time (if available)."]
        created_at: Option<DateTime<FixedOffset>>
    }

    transparent_property! {
        #[doc = "Floating IP description."]
        description: ref Option<String>
    }

    transparent_property! {
        #[doc = "DNS domain for the floating IP (if available)."]
        dns_domain: ref Option<String>
    }

    transparent_property! {
        #[doc = "DNS domain for the floating IP (if available)."]
        dns_name: ref Option<String>
    }

    transparent_property! {
        #[doc = "IP address of the port associated with the IP (if any)."]
        fixed_ip_address: Option<net::IpAddr>
    }

    transparent_property! {
        #[doc = "Floating IP address (if allocated)."]
        floating_ip_address: Option<net::IpAddr>
    }

    transparent_property! {
        #[doc = "Unique ID."]
        id: ref String
    }

    transparent_property! {
        #[doc = "Status of the floating IP."]
        status: protocol::FloatingIpStatus
    }

    transparent_property! {
        #[doc = "Last update data and time (if available)."]
        updated_at: Option<DateTime<FixedOffset>>
    }
}
