var searchIndex = {};
searchIndex['netaddr'] = {"items":[[0,"","netaddr",""],[0,"addr","","Provide operations over IP addresses."],[2,"IpAddr","netaddr::addr","Describe an IP address"],[12,"Ipv4Addr","","",0],[12,"Ipv6Addr","","",0],[2,"IpAddrVersion","","Describe the version of an IP address."],[12,"Ipv4","","",1],[12,"Ipv6","","",1],[0,"ipv4","","Provide operations over IPv4 addresses."],[1,"IpAddr","netaddr::addr::ipv4",""],[18,"MAX_PREFIXLEN","",""],[10,"decode","","",2],[10,"encode","","",2],[10,"hash","","",2],[10,"eq","","",2],[10,"ne","","",2],[10,"clone","","",2],[10,"version","","Get the corresponding IP address version.",2],[10,"max_prefixlen","","The total number of bits in the address representation for this version: `32` for IPv4, `128` for IPv6.",2],[10,"with_prefixlen","","Create an IP mask with the specified prefixlen.",2],[10,"packed","","The binary representation of this address - a bytes vector of the appropriate length (most significant octet first).\nThis is 4 bytes for IPv4 and 16 bytes for IPv6.",2],[10,"from_u32","","Create an `IpAddr` instance from a 32-bits integer.",2],[10,"to_u32","","Convert an `IpAddr` instance to a 32-bits integer.",2],[10,"add","","",2],[10,"sub","","",2],[10,"bitxor","","",2],[10,"bitor","","",2],[10,"bitand","","",2],[10,"not","","",2],[10,"partial_cmp","","",2],[10,"cmp","","",2],[10,"from_std","","Create an `ipv4::IpAddr` instance from a Rust's standard library `IpAddr` instance.",2],[10,"to_std","","Create a Rust's standard library `IpAddr` instance from an `ipv4::IpAddr` instance.",2],[10,"into_std","","Convert an `ipv4::IpAddr` instance into a Rust's standard library `IpAddr` instance.",2],[10,"fmt","","",2],[10,"from_str","","",2],[0,"ipv6","netaddr::addr","Provide operations over IPv6 addresses."],[1,"IpAddr","netaddr::addr::ipv6",""],[18,"MAX_PREFIXLEN","",""],[10,"decode","","",3],[10,"encode","","",3],[10,"hash","","",3],[10,"eq","","",3],[10,"ne","","",3],[10,"clone","","",3],[10,"version","","Get the corresponding IP address version.",3],[10,"max_prefixlen","","The total number of bits in the address representation for this version: `32` for IPv4, `128` for IPv6.",3],[10,"with_prefixlen","","Create an IP mask with the specified prefixlen.",3],[10,"packed","","The binary representation of this address - a bytes vector of the appropriate length (most significant octet first).\nThis is 4 bytes for IPv4 and 16 bytes for IPv6.",3],[10,"from_u128","","Create an `IpAddr` instance from a 128-bits integer.",3],[10,"to_u128","","Convert an `IpAddr` instance to a 128-bits integer.",3],[10,"from_simd","","Create an `IpAddr` instance from a 128-bits SIMD integer.",3],[10,"to_simd","","Convert an `IpAddr` instance to a 128-bits SIMD integer.",3],[10,"add","","",3],[10,"sub","","",3],[10,"bitxor","","> Use SIMD to do operations on 128-bits integer.",3],[10,"bitor","","> Use SIMD to do operations on 128-bits integer.",3],[10,"bitand","","> Use SIMD to do operations on 128-bits integer.",3],[10,"not","","",3],[10,"partial_cmp","","",3],[10,"cmp","","",3],[10,"from_std","","Create an `ipv6::IpAddr` instance from a Rust's standard library `IpAddr` instance.",3],[10,"to_std","","Create a Rust's standard library `IpAddr` instance from an `ipv6::IpAddr` instance.",3],[10,"into_std","","Convert an `ipv6::IpAddr` instance into a Rust's standard library `IpAddr` instance.",3],[10,"fmt","","",3],[10,"from_str","","",3],[10,"decode","netaddr::addr","",0],[10,"encode","","",0],[10,"hash","","",0],[10,"cmp","","",0],[10,"partial_cmp","","",0],[10,"lt","","",0],[10,"le","","",0],[10,"gt","","",0],[10,"ge","","",0],[10,"eq","","",0],[10,"ne","","",0],[10,"clone","","",0],[10,"decode","","",1],[10,"encode","","",1],[10,"hash","","",1],[10,"eq","","",1],[10,"ne","","",1],[10,"fmt","","",1],[10,"version","","Get the corresponding IP address version.",0],[10,"max_prefixlen","","The total number of bits in the address representation for this version: `32` for IPv4, `128` for IPv6.",0],[10,"with_prefixlen","","Create an IP mask with the specified prefixlen.",0],[10,"packed","","The binary representation of this address - a bytes vector of the appropriate length (most significant octet first).\nThis is 4 bytes for IPv4 and 16 bytes for IPv6.",0],[10,"add","","",0],[10,"sub","","",0],[10,"bitxor","","",0],[10,"bitor","","",0],[10,"bitand","","",0],[10,"not","","",0],[10,"from_std","","Create an `IpAddr` instance from a Rust's standard library `IpAddr` instance.",0],[10,"to_std","","Create a Rust's standard library `IpAddr` instance from an `IpAddr` instance.",0],[10,"into_std","","Convert an `IpAddr` instance into a Rust's standard library `IpAddr` instance.",0],[10,"fmt","","",0],[10,"from_str","","",0],[0,"net","netaddr","Provide operations over IP networks."],[1,"Hosts","netaddr::net",""],[2,"IpNetwork","","Describe an IP network."],[12,"Ipv4Network","","",4],[12,"Ipv6Network","","",4],[0,"ipv4","","Provide operations over IPv4 networks."],[1,"IpNetwork","netaddr::net::ipv4",""],[1,"Hosts","",""],[10,"decode","","",5],[10,"encode","","",5],[10,"hash","","",5],[10,"eq","","",5],[10,"ne","","",5],[10,"clone","","",5],[10,"version","","Get the corresponding IP address version.",5],[10,"address","","Get the network address for the network.",5],[10,"broadcast_address","","Get the broadcast address for the network.",5],[10,"prefix","","Get the length of the network prefix, in bits.",5],[10,"host_prefix","","Get the length of the host prefix, in bits.",5],[10,"num_addresses","","The total number of addresses in the network.",5],[10,"mask","","Get the mask of the network.",5],[10,"range","","Get the hosts range this network have.",5],[10,"contains","","`true` if this ip is contained in the network.",5],[10,"overlaps","","`true` if this network is partly or wholly contained in other or other is wholly contained in this network.",5],[10,"iter","","Iterate over all addresses of this network.",5],[10,"hosts_iter","","Iterate over all usable hosts of this network.",5],[10,"partial_cmp","","",5],[10,"cmp","","",5],[10,"fmt","","",5],[10,"clone","","",6],[10,"next","","",6],[10,"next_back","","",6],[0,"ipv6","netaddr::net","Provide operations over IPv6 networks."],[1,"IpNetwork","netaddr::net::ipv6",""],[1,"Hosts","",""],[10,"decode","","",7],[10,"encode","","",7],[10,"hash","","",7],[10,"eq","","",7],[10,"ne","","",7],[10,"clone","","",7],[10,"version","","Get the corresponding IP address version.",7],[10,"address","","Get the network address for the network.",7],[10,"broadcast_address","","Get the broadcast address for the network.",7],[10,"prefix","","Get the length of the network prefix, in bits.",7],[10,"host_prefix","","Get the length of the host prefix, in bits.",7],[10,"num_addresses","","The total number of addresses in the network.",7],[10,"mask","","Get the mask of the network.",7],[10,"range","","Get the hosts range this network have.",7],[10,"contains","","`true` if this ip is contained in the network.",7],[10,"overlaps","","`true` if this network is partly or wholly contained in other or other is wholly contained in this network.",7],[10,"iter","","Iterate over all addresses of this network.",7],[10,"hosts_iter","","Iterate over all usable hosts of this network.",7],[10,"partial_cmp","","",7],[10,"cmp","","",7],[10,"fmt","","",7],[10,"clone","","",8],[10,"next","","",8],[10,"next_back","","",8],[10,"decode","netaddr::net","",4],[10,"encode","","",4],[10,"hash","","",4],[10,"cmp","","",4],[10,"partial_cmp","","",4],[10,"lt","","",4],[10,"le","","",4],[10,"gt","","",4],[10,"ge","","",4],[10,"eq","","",4],[10,"ne","","",4],[10,"clone","","",4],[10,"version","","Get the corresponding IP address version.",4],[10,"address","","Get the network address for the network.",4],[10,"broadcast_address","","Get the broadcast address for the network.",4],[10,"prefix","","Get the length of the network prefix, in bits.",4],[10,"host_prefix","","Get the length of the host prefix, in bits.",4],[10,"num_addresses","","The total number of addresses in the network.",4],[10,"mask","","Get the mask of the network.",4],[10,"range","","Get the hosts range this network have.",4],[10,"contains","","`true` if this ip is contained in the network.",4],[10,"overlaps","","`true` if this network is partly or wholly contained in other or other is wholly contained in this network.",4],[10,"iter","","Iterate over all addresses of this network.",4],[10,"hosts_iter","","Iterate over all usable hosts of this network.",4],[10,"fmt","","",4],[10,"clone","","",9],[10,"next","","",9],[10,"next_back","","",9]],"paths":[[2,"IpAddr"],[2,"IpAddrVersion"],[1,"IpAddr"],[1,"IpAddr"],[2,"IpNetwork"],[1,"IpNetwork"],[1,"Hosts"],[1,"IpNetwork"],[1,"Hosts"],[1,"Hosts"]]};
initSearch(searchIndex);
