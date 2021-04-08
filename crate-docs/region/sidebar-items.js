initSidebarItems({"enum":[["Error","A collection of possible errors."]],"fn":[["alloc","Allocates one or more pages of memory, with a defined protection."],["alloc_at","Allocates one or more pages of memory, at a specific address, with a defined protection."],["lock","Locks one or more memory regions to RAM."],["protect","Changes the memory protection of one or more pages."],["protect_with_handle","Temporarily changes the memory protection of one or more pages."],["query","Queries the OS with an address, returning the region it resides within."],["query_range","Queries the OS for mapped regions that overlap with the specified range."],["unlock","Unlocks one or more memory regions from RAM."]],"mod":[["page","Page related functions."]],"struct":[["Allocation","A handle to an owned region of memory."],["LockGuard","A RAII implementation of a scoped lock."],["ProtectGuard","A RAII implementation of a scoped protection guard."],["Protection","A bitflag of zero or more protection attributes."],["QueryIter","An iterator over the [`Region`]s that encompass an address range."],["Region","A descriptor for a mapped memory region."]],"type":[["Result","The result type used by this library."]]});