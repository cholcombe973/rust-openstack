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

//! Network API implementation bits.

mod base;
mod networks;
mod ports;
mod protocol;
mod subnets;

pub use self::networks::{Network, NetworkQuery};
pub use self::ports::{NewPort, Port, PortIpAddress, PortIpRequest, PortQuery};
pub use self::protocol::{AllocationPool, HostRoute, Ipv6Mode, IpVersion,
                         NetworkStatus, NetworkSortKey, PortExtraDhcpOption,
                         PortSortKey, SubnetSortKey};
pub use self::subnets::{Subnet, SubnetQuery};
