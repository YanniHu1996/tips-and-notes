
It communicates with the consensus modules on other
servers to ensure that every log eventually contains the
same requests in the same order,


### List key properties of the algorithm

the key safety property for Raft is the State Machine Safety Property


Election Safety: at most one leader can be elected in a given term. 

Leader Append-Only: a leader never overwrites or deletes entries in its log; it only appends new entries. §5.3

Log Matching: if two logs contain an entry with the same index and term, then the logs are identical in all entries up through the given index. §5.3

Leader Completeness: if a log entry is committed in a
given term, then that entry will be present in the logs
of the leaders for all higher-numbered terms. §5.4

State Machine Safety: if a server has applied a log entry
at a given index to its state machine, no other server
will ever apply a different log entry for the same index.

### Reference 

https://raft.github.io/raft.pdf
