## Setting Up High Availability PostgreSQL with One Primary and Two Standby Instances

High availability (HA) is a crucial aspect of modern database systems, ensuring minimal downtime and data loss. PostgreSQL, a powerful open-source relational database, provides robust features for HA. In this blog, we'll walk through setting up a high availability PostgreSQL cluster with one primary and two standby instances.

### Prerequisites

Before diving into the setup, ensure the following:

1. PostgreSQL 16 is installed on all three servers.
2. Network settings are configured to allow communication between the primary and standby instances.
3. SSH access is set up between servers for file transfer and command execution.

### Step 1: Initialize the Primary Server

First, we need to initialize and configure the primary PostgreSQL server.

1. **Initialize the primary database cluster:**
   ```bash
   initdb -D /var/lib/pgsql/16/data
   ```

2. **Configure the primary server:**
   Edit the `postgresql.conf` file, typically located in `/var/lib/pgsql/16/data`:
   ```ini
   listen_addresses = '*'
   wal_level = replica
   max_wal_senders = 10
   wal_keep_size = 16MB
   hot_standby = on
   ```
   Update the `pg_hba.conf` file to allow replication connections from the standby servers:
   ```ini
   host replication all [standby_server_ip/32] md5
   ```

3. **Start the primary server:**
   ```bash
   pg_ctl -D /var/lib/pgsql/16/data start
   ```

4. **Create a replication user:**
   ```sql
   CREATE ROLE replicator WITH REPLICATION PASSWORD 'your_password' LOGIN;
   ```

### Step 2: Set Up Standby Servers

Next, we'll set up the standby servers to replicate the primary server.

1. **Initialize the standby database clusters:**
   ```bash
   pg_basebackup -h [primary_server_ip] -D /var/lib/pgsql/16/data -U replicator -P -R
   ```
   The `-R` flag creates a `recovery.conf` file in the standby data directory with necessary settings to follow the primary server.

2. **Configure standby servers:**
   Ensure the `postgresql.conf` file for the standby servers includes:
   ```ini
   hot_standby = on
   ```
   The `recovery.conf` file (automatically created by `pg_basebackup` with the `-R` flag) should contain:
   ```ini
   standby_mode = 'on'
   primary_conninfo = 'host=[primary_server_ip] port=5432 user=replicator password=your_password'
   ```

3. **Start the standby servers:**
   ```bash
   pg_ctl -D /var/lib/pgsql/16/data start
   ``*

### Step 3: Verify the Setup

After setting up the primary and standby servers, we need to verify the configuration.

1. **Check the primary server status:**
   ```sql
   SELECT * FROM pg_stat_replication;
   ```

2. **Check the standby servers status:**
   ```bash
   ps -ef | grep postgres
   ```

### Step 4: Automate Failover and Monitoring

For a more robust HA setup, consider using tools like **Patroni**, **repmgr**, or **Pacemaker** to automate failover and manage your cluster. Here’s a brief outline for using Patroni:

1. **Install Patroni** on all servers.
2. **Configure Patroni** on each server, specifying the role (primary or standby) and connection details.
3. **Set up etcd, Consul, or Zookeeper** for distributed consensus.
4. **Start Patroni** on all servers to manage the PostgreSQL instances.

### Step 5: Additional Configuration and Best Practices

1. **Backup Configuration:**
   Set up regular backups using tools like `pg_dump` or `pgBackRest`.

2. **Monitoring:**
   Use monitoring tools like `pgMonitor`, `Prometheus`, and `Grafana` to keep an eye on the health and performance of your PostgreSQL cluster.

3. **Security:**
   Ensure secure communication between nodes using SSL/TLS.

### Understanding the Differences Between Primary and Standby Servers

#### Role and Responsibilities

- **Primary Server:** Acts as the main read-write instance, handling all write operations and generating WAL (Write-Ahead Logging) files for standbys.
- **Standby Servers:** Serve as read-only replicas, applying WAL files from the primary to stay in sync. Can be promoted to primary in case of primary server failure.

#### Configuration Differences

- **Primary Server (`postgresql.conf`):**
   ```ini
   listen_addresses = '*'
   wal_level = replica
   max_wal_senders = 10
   wal_keep_size = 16MB
   hot_standby = on
   ```

- **Standby Server (`postgresql.conf`):**
   ```ini
   hot_standby = on
   ```

- **Standby Server (`recovery.conf` or `standby.signal`):**
   ```ini
   standby_mode = 'on'
   primary_conninfo = 'host=[primary_server_ip] port=5432 user=replicator password=your_password'
   ```

#### Write Operations

- **Primary Server:** Handles all write operations directly, generating WAL entries.
- **Standby Server:** Does not handle write operations directly; applies WAL entries from the primary server.

#### Failover and Recovery

- **Primary Server:** Continues normal operation unless a failure occurs. In case of failure, a standby server must be promoted to primary.
- **Standby Server:** Can be promoted to primary if the current primary fails. Starts accepting read-write operations upon promotion.

By following these steps, you'll have a basic HA PostgreSQL cluster with one primary and two standby servers. For production environments, it's recommended to implement additional tools and practices to ensure reliability and seamless failover.

### Conclusion

Setting up a high availability PostgreSQL cluster ensures your database remains available and resilient to failures. By carefully configuring the primary and standby servers, and using automation and monitoring tools, you can achieve a robust and reliable HA PostgreSQL setup. Happy clustering!
