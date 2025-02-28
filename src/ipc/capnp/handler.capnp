# Copyright (c) 2021 The Bitcoin Core developers
# Distributed under the MIT software license, see the accompanying
# file COPYING or http://www.opensource.org/licenses/mit-license.php.

@0xb5a9256c00627b30;

using Cxx = import "c++.capnp";
$Cxx.namespace("ipc::capnp::messages");

using Proxy = import "proxy.capnp";
$Proxy.include("interfaces/handler.h");
$Proxy.includeTypes("ipc/capnp/handler-types.h");

interface Handler $Proxy.wrap("interfaces::Handler") {
    destroy @0 (context :Proxy.Context) -> ();
    disconnect @1 (context :Proxy.Context) -> ();
}