# PostgreSQL Schema & Extension Management Guide  
**Best Practices for Security, Permissions, and Workflows**  

---

## 1. **Schemas in PostgreSQL**  

### 1.1 Default Schemas  
- **`public` Schema**:  
  - The default schema for user-created objects (tables, functions, extensions).  
  - Included in the schema search path (`search_path = "$user", public, pg_catalog`).  
- **`pg_catalog` Schema**:  
  - Contains system catalog tables, data types, and built-in functions.  
  - Always present in the search path but **not intended for user-created objects**.  

### 1.2 Key Rules for Extensions  
- Extensions are installed in the schema specified in `CREATE EXTENSION` or the first schema in the user’s `search_path` (usually `public`).  
- **Never install extensions directly into `pg_catalog`** to avoid conflicts with system objects.  

### 1.3 Schema Management Best Practices  
```sql  
-- Explicitly specify a schema for extensions:  
CREATE SCHEMA extensions;  
CREATE EXTENSION pg_stat_statements SCHEMA extensions;  

-- Modify a user’s search path to prioritize schemas:  
ALTER ROLE streaming_user SET search_path = extensions, public, pg_catalog;  
```  

---

## 2. **Trusted vs. Untrusted Extensions**  

### 2.1 Definitions  
- **Trusted Extensions**:  
  - Marked as `trusted = true` in their `.control` file.  
  - Safe for non-superusers to install (e.g., `pgcrypto`, `hstore`).  
- **Untrusted Extensions**:  
  - Marked as `trusted = false` in their `.control` file.  
  - Require superuser privileges (e.g., `plperlu`, `postgres_fdw`, `file_fdw`).  

### 2.2 Security Implications  
- **Trusted Extensions**: Limited to safe operations (e.g., cryptographic functions).  
- **Untrusted Extensions**: Can execute arbitrary code or access external systems.  

### 2.3 Identifying Extension Types  
```sql  
-- List all available extensions and trust status:  
SELECT name, comment, trusted FROM pg_available_extensions;  
```  

---

## 3. **Permissions & Workflows**  

### 3.1 Installing Untrusted Extensions  
- **Superuser Requirement**:  
  Non-superusers (e.g., `streaming_user`) **cannot** install untrusted extensions by default.  

#### Workaround (Without Granting Superuser):  
```sql  
-- As a superuser:  
CREATE SCHEMA secure_extensions;  
CREATE EXTENSION postgres_fdw SCHEMA secure_extensions;  

-- Grant limited privileges to streaming_user:  
GRANT USAGE ON SCHEMA secure_extensions TO streaming_user;  
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA secure_extensions TO streaming_user;  
```  

### 3.2 Granting Extension Permissions  
- For trusted extensions, grant `CREATE` privilege on the target schema:  
```sql  
GRANT CREATE ON SCHEMA public TO streaming_user;  
```  

---

## 4. **Best Practices**  

### 4.1 Schema Isolation  
- Use dedicated schemas for extensions (e.g., `extensions`, `monitoring`) to avoid clutter in `public`.  

### 4.2 Security Audits  
- Review extensions’ `.control` files (located in `SHAREDIR/extension/`) to verify trust status.  
- Audit installed extensions regularly:  
```sql  
SELECT * FROM pg_extension;  
```  

### 4.3 Least-Privilege Principle  
- Avoid granting `SUPERUSER` or `CREATE` privileges unless absolutely necessary.  

---

## 5. **Troubleshooting Common Issues**  

| **Issue**                          | **Solution**                                                                 |  
|------------------------------------|-----------------------------------------------------------------------------|  
| `ERROR: permission denied to create extension` | Ensure the user has `CREATE` privilege on the schema or is a superuser. |  
| `Extension "XYZ" not found`        | Verify the extension is installed on the server using `pg_available_extensions`. |  
| `Function not found in schema`     | Check the user’s `search_path` and grant `USAGE` on the extension’s schema. |  

---

## 6. **Example Workflow**  
**Goal**: Allow `streaming_user` to use `postgres_fdw` (untrusted) without superuser access.  

1. **As Superuser**:  
```sql  
CREATE SCHEMA fdw_schema;  
CREATE EXTENSION postgres_fdw SCHEMA fdw_schema;  
GRANT USAGE ON SCHEMA fdw_schema TO streaming_user;  
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA fdw_schema TO streaming_user;  
```  

2. **As `streaming_user`**:  
```sql  
-- Use the extension’s functions:  
CREATE SERVER remote_server FOREIGN DATA WRAPPER fdw_schema.postgres_fdw;  
```  

---

## 7. **Conclusion**  
- Always prefer `public` or dedicated schemas over `pg_catalog` for extensions.  
- Restrict untrusted extensions to superusers and use schemas to isolate privileges.  
- Regularly audit extensions and adhere to the principle of least privilege.  

**Reference**: [PostgreSQL Extension Documentation](https://www.postgresql.org/docs/current/extend-extensions.html)
