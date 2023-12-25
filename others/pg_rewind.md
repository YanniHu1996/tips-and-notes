 `pg_rewind` is a tool provided with PostgreSQL that is used to synchronize a PostgreSQL database cluster with another copy of the same cluster. This is particularly useful in scenarios where you have a standby server that has diverged from the primary server due to a failover or other reasons.

The basic idea behind `pg_rewind` is that it makes it possible to bring a former master server back online as a standby without requiring a full base backup. This can be a significant time saver, especially with large databases.

Here's a simplified example of how `pg_rewind` is typically used:

**Scenario**: Imagine you have a primary PostgreSQL server (`Server A`) and a standby server (`Server B`). Due to a network issue, `Server B` gets isolated and then is promoted to a primary server. Later, when the network issue is resolved, you want to reintegrate `Server B` back as a standby server to the original primary (`Server A`), which is still operational.

**Steps**:

1. **Ensure Server B is Stopped**: Before you can use `pg_rewind`, the server that needs to be synchronized (`Server B` in our case) must be cleanly shut down.

2. **Run pg_rewind**:
   - You would execute `pg_rewind` on `Server B`, using the data directory of `Server B` and connecting to the current primary server (`Server A`).

    Example command:
    ```bash
    pg_rewind --target-pgdata=/path/to/server_b/data --source-server='host=server_a_host port=5432 user=your_user dbname=your_db'
    ```

   In this command:
   - `--target-pgdata` is the data directory of the server to be synchronized (`Server B`).
   - `--source-server` specifies the connection string to the current primary server (`Server A`).

3. **Restart Server B as Standby**: After `pg_rewind` completes, you can start `Server B` as a standby server. It will now be in sync with `Server A` and can continue replication from the point where `pg_rewind` synchronized it.

**Important Notes**:

- `pg_rewind` requires that the servers have been set up with WAL archiving or streaming replication, as it relies on WAL records to synchronize the data.
- Both servers must have been running the same major version of PostgreSQL.
- It's important to ensure proper backups and to understand the current state of your clusters before using tools like `pg_rewind`, as incorrect usage can lead to data loss.

Remember, this is a simplified example. In real-world scenarios, additional considerations like network configurations, authentication, and specific PostgreSQL settings will also play a role.


## Real-world example

Creating a detailed, real-world example involving `pg_rewind` requires considering several components, including initial setup, failover, and recovery. I'll outline a scenario with two PostgreSQL servers: a primary (Server A) and a standby (Server B), and demonstrate how `pg_rewind` is used when the standby server has been promoted and then needs to be reintegrated.

### Initial Setup

1. **Environment**:
   - Server A (Primary): IP `192.168.1.1`
   - Server B (Standby): IP `192.168.1.2`
   - PostgreSQL version: 12 (ensure both servers run the same major version)
   - Operating System: Linux-based

2. **Configure PostgreSQL on Both Servers**:
   - Enable WAL archiving and replication settings in `postgresql.conf` on Server A:
     ```
     wal_level = replica
     archive_mode = on
     archive_command = 'cp %p /path/to/wal_archive/%f'
     max_wal_senders = 3
     ```

   - Adjust `pg_hba.conf` on Server A to allow replication connections from Server B:
     ```
     host replication all 192.168.1.2/32 md5
     ```

3. **Initial Data Copy**:
   - On Server A, create a base backup:
     ```
     pg_basebackup -h 192.168.1.1 -D /var/lib/postgresql/12/main -U replicator -P -v
     ```
   - This command uses a replication user `replicator` to copy the data directory to Server B.

4. **Configure Standby Server (Server B)**:
   - Create a `recovery.conf` (or use `standby.signal` in later PostgreSQL versions) with connection info to Server A:
     ```
     standby_mode = 'on'
     primary_conninfo = 'host=192.168.1.1 port=5432 user=replicator password=your_password'
     ```

5. **Start Both Servers**:
   - Start PostgreSQL on both servers. Server B will start in standby mode and begin replicating from Server A.

### Failover and Promotion of Standby

1. **Simulate a Failover**:
   - Assume Server A goes down due to a network failure.

2. **Promote Server B to Primary**:
   - Run `pg_ctl promote` on Server B to promote it to a primary server.

### Reintegrating Server B as Standby using `pg_rewind`

1. **Network Restoration and Initial Checks**:
   - When the network issue is resolved, ensure Server A is running and accessible.
   - Ensure Server B (now acting as a primary) is stopped cleanly.

2. **Use pg_rewind**:
   - On Server B, run `pg_rewind`:
     ```
     pg_rewind --target-pgdata=/var/lib/postgresql/12/main --source-server='host=192.168.1.1 port=5432 user=replicator password=your_password'
     ```
   - This command rewinds the Server B cluster to match the Server A cluster.

3. **Reconfigure Server B as Standby**:
   - Adjust the `recovery.conf` or `standby.signal` file on Server B again to follow Server A.

4. **Start Server B as Standby**:
   - Start PostgreSQL on Server B. It will now follow Server A as a standby.

### Notes

- **Authentication**: Ensure the `replicator` user has replication privileges and is present in `pg_hba.conf` of both servers.
- **Networking**: Proper network configurations and firewall rules should be in place to allow communication between the servers on the required ports (usually 5432 for PostgreSQL).
- **Data Safety**: Always back up your data before performing operations like `pg_rewind`, as there's a risk of data loss if not done correctly.
- **Version Compatibility**: `pg_rewind` and the replication setup can vary between PostgreSQL versions, so adjust configurations accordingly.

This example gives a general idea of how `pg_rewind` can be used in a real-world scenario. However, every environment can have its unique challenges and requirements, so adjustments may be necessary to fit specific needs.
