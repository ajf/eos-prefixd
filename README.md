eos-prefixd
===========

Run a DHCPv6 client to get a DHCPv6 Prefix Delegation (IA_PD) and assign it to downstream SVIs on EOS.

This uses gNMI interface to Arista to add/delete interfaces.

It will not configure RA parameters since it doesn't appear the EOS supports RA configuration via gNMI.

29-Dec-2025
-----------

Not functional, doesn't work, don't try to use it unless you want to write code
