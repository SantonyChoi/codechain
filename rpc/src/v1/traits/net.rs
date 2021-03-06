// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use jsonrpc_core::Result;
use primitives::H256;

build_rpc_trait! {
    pub trait Net {
        # [rpc(name = "net_shareSecret")]
        fn share_secret(&self, H256, ::std::net::IpAddr, u16) -> Result<()>;

        # [rpc(name = "net_connect")]
        fn connect(&self, ::std::net::IpAddr, u16) -> Result<()>;

        # [rpc(name = "net_disconnect")]
        fn disconnect(&self, ::std::net::IpAddr, u16) -> Result<()>;

        # [rpc(name = "net_isConnected")]
        fn is_connected(&self, ::std::net::IpAddr, u16) -> Result<bool>;
    }
}
