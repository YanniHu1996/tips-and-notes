

## Create DOMAIN
The SQL statement [`CREATE DOMAIN bdr.crdt_delta_counter AS bigint NOT NULL DEFAULT 0;`](command:_github.copilot.openSymbolFromReferences?%5B%22CREATE%20DOMAIN%20bdr.crdt_delta_counter%20AS%20bigint%20NOT%20NULL%20DEFAULT%200%3B%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A14%7D%7D%5D%5D "Go to definition") is used to create a domain in PostgreSQL. A domain is essentially a custom data type with constraints. Here's a detailed explanation:

### Breakdown of the Statement

1. **CREATE DOMAIN**:
   - The [`CREATE DOMAIN`](command:_github.copilot.openSymbolFromReferences?%5B%22CREATE%20DOMAIN%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A7%7D%7D%5D%5D "Go to definition") command is used to define a new domain. A domain is a user-defined data type that can include constraints such as [`NOT NULL`](command:_github.copilot.openSymbolFromReferences?%5B%22NOT%20NULL%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A51%7D%7D%5D%5D "Go to definition"), `CHECK`, and default values.

2. **bdr.crdt_delta_counter**:
   - This specifies the name of the domain. In this case, the domain is named [`crdt_delta_counter`](command:_github.copilot.openSymbolFromReferences?%5B%22crdt_delta_counter%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A18%7D%7D%5D%5D "Go to definition") and it is created within the [`bdr`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A14%7D%7D%5D%5D "Go to definition") schema. The [`bdr`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A14%7D%7D%5D%5D "Go to definition") schema is likely a namespace used to organize related database objects.

3. **AS bigint**:
   - This specifies the base data type for the domain. Here, the domain is based on the [`bigint`](command:_github.copilot.openSymbolFromReferences?%5B%22bigint%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A40%7D%7D%5D%5D "Go to definition") data type, which is a 64-bit integer.

4. **NOT NULL**:
   - This constraint ensures that any value assigned to the domain cannot be [`NULL`](command:_github.copilot.openSymbolFromReferences?%5B%22NULL%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A51%7D%7D%5D%5D "Go to definition"). Every value must be a valid [`bigint`](command:_github.copilot.openSymbolFromReferences?%5B%22bigint%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A40%7D%7D%5D%5D "Go to definition").

5. **DEFAULT 0**:
   - This sets the default value for the domain. If no value is provided when a column of this domain type is inserted, it will default to [`0`](command:_github.copilot.openSymbolFromReferences?%5B%220%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A64%7D%7D%5D%5D "Go to definition").

### Full Explanation

The statement [`CREATE DOMAIN bdr.crdt_delta_counter AS bigint NOT NULL DEFAULT 0;`](command:_github.copilot.openSymbolFromReferences?%5B%22CREATE%20DOMAIN%20bdr.crdt_delta_counter%20AS%20bigint%20NOT%20NULL%20DEFAULT%200%3B%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A14%7D%7D%5D%5D "Go to definition") creates a custom data type named [`crdt_delta_counter`](command:_github.copilot.openSymbolFromReferences?%5B%22crdt_delta_counter%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A18%7D%7D%5D%5D "Go to definition") within the [`bdr`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A14%7D%7D%5D%5D "Go to definition") schema. This custom type is based on the [`bigint`](command:_github.copilot.openSymbolFromReferences?%5B%22bigint%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A40%7D%7D%5D%5D "Go to definition") data type and includes the following constraints:

- **NOT NULL**: Values of this type cannot be [`NULL`](command:_github.copilot.openSymbolFromReferences?%5B%22NULL%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A51%7D%7D%5D%5D "Go to definition").
- **DEFAULT 0**: The default value for this type is [`0`](command:_github.copilot.openSymbolFromReferences?%5B%220%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A64%7D%7D%5D%5D "Go to definition").

### Example Usage

Here’s how you might use this domain in a table definition:

```sql
-- Create a table using the custom domain
CREATE TABLE example_table (
    id serial PRIMARY KEY,
    delta bdr.crdt_delta_counter
);

-- Insert a row without specifying the delta value
-- The delta column will default to 0
INSERT INTO example_table (id) VALUES (1);

-- Insert a row with a specific delta value
INSERT INTO example_table (id, delta) VALUES (2, 10);

-- Query the table
SELECT * FROM example_table;
```

### Benefits of Using Domains

- **Consistency**: By defining a domain, you ensure that the same constraints are applied consistently across multiple tables.
- **Reusability**: You can reuse the domain in multiple table definitions without having to redefine the constraints each time.
- **Maintainability**: If you need to change the constraints, you can do so in one place (the domain definition) rather than in every table definition.

In summary, the statement [`CREATE DOMAIN bdr.crdt_delta_counter AS bigint NOT NULL DEFAULT 0;`](command:_github.copilot.openSymbolFromReferences?%5B%22CREATE%20DOMAIN%20bdr.crdt_delta_counter%20AS%20bigint%20NOT%20NULL%20DEFAULT%200%3B%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3511%2C%22character%22%3A14%7D%7D%5D%5D "Go to definition") creates a custom data type with specific constraints, making it easier to enforce data integrity and consistency across your database schema.

## CREATE EVENT TRIGGER

The SQL statement creates an event trigger in PostgreSQL. Here's a detailed explanation:

### Breakdown of the Statement

1. **CREATE EVENT TRIGGER**:
   - The [`CREATE EVENT TRIGGER`](command:_github.copilot.openSymbolFromReferences?%5B%22CREATE%20EVENT%20TRIGGER%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3657%2C%22character%22%3A13%7D%7D%5D%5D "Go to definition") command is used to define a new event trigger. Event triggers are special types of triggers that fire in response to certain database events, such as DDL (Data Definition Language) commands.

2. **bdr_disallow_alter_table_rewrite_trg**:
   - This is the name of the event trigger being created. In this case, the trigger is named [`bdr_disallow_alter_table_rewrite_trg`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr_disallow_alter_table_rewrite_trg%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3657%2C%22character%22%3A21%7D%7D%5D%5D "Go to definition").

3. **ON table_rewrite**:
   - This specifies the event that will cause the trigger to fire. The [`table_rewrite`](command:_github.copilot.openSymbolFromReferences?%5B%22table_rewrite%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3658%2C%22character%22%3A3%7D%7D%5D%5D "Go to definition") event occurs when a table is rewritten, which can happen during certain `ALTER TABLE` operations that require a full table rewrite.

4. **EXECUTE FUNCTION bdr.disallow_alter_table_rewrite()**:
   - This specifies the function that will be executed when the event trigger fires. In this case, the function [`bdr.disallow_alter_table_rewrite()`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr.disallow_alter_table_rewrite()%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3659%2C%22character%22%3A17%7D%7D%5D%5D "Go to definition") from the [`bdr`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3659%2C%22character%22%3A17%7D%7D%5D%5D "Go to definition") schema will be executed.

### Full Explanation

The statement creates an event trigger named [`bdr_disallow_alter_table_rewrite_trg`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr_disallow_alter_table_rewrite_trg%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3657%2C%22character%22%3A21%7D%7D%5D%5D "Go to definition") that fires on the [`table_rewrite`](command:_github.copilot.openSymbolFromReferences?%5B%22table_rewrite%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3658%2C%22character%22%3A3%7D%7D%5D%5D "Go to definition") event. When this event occurs, the trigger will execute the function [`bdr.disallow_alter_table_rewrite()`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr.disallow_alter_table_rewrite()%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3659%2C%22character%22%3A17%7D%7D%5D%5D "Go to definition").

### Purpose

The purpose of this event trigger is likely to prevent or handle `ALTER TABLE` operations that would cause a table rewrite. The function [`bdr.disallow_alter_table_rewrite()`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr.disallow_alter_table_rewrite()%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3659%2C%22character%22%3A17%7D%7D%5D%5D "Go to definition") would contain the logic to either disallow such operations or take specific actions when they are attempted.

### Example Scenario

1. **Creating the Event Trigger**:
   ```sql
   CREATE EVENT TRIGGER bdr_disallow_alter_table_rewrite_trg
   ON table_rewrite
   EXECUTE FUNCTION bdr.disallow_alter_table_rewrite();
   ```

2. **Function Definition**:
   The function [`bdr.disallow_alter_table_rewrite()`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr.disallow_alter_table_rewrite()%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3659%2C%22character%22%3A17%7D%7D%5D%5D "Go to definition") would need to be defined in the [`bdr`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3659%2C%22character%22%3A17%7D%7D%5D%5D "Go to definition") schema. Here’s an example of what the function might look like:

   ```sql
   CREATE OR REPLACE FUNCTION bdr.disallow_alter_table_rewrite()
   RETURNS event_trigger AS $$
   BEGIN
       RAISE EXCEPTION 'ALTER TABLE operations that cause a table rewrite are not allowed.';
   END;
   $$ LANGUAGE plpgsql;
   ```

3. **Trigger Firing**:
   When an `ALTER TABLE` operation that requires a table rewrite is attempted, the event trigger will fire, and the function [`bdr.disallow_alter_table_rewrite()`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr.disallow_alter_table_rewrite()%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3659%2C%22character%22%3A17%7D%7D%5D%5D "Go to definition") will be executed, raising an exception and preventing the operation.

### Summary

The statement:
```sql
CREATE EVENT TRIGGER bdr_disallow_alter_table_rewrite_trg
ON table_rewrite
EXECUTE FUNCTION bdr.disallow_alter_table_rewrite();
```
creates an event trigger that fires on the [`table_rewrite`](command:_github.copilot.openSymbolFromReferences?%5B%22table_rewrite%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3658%2C%22character%22%3A3%7D%7D%5D%5D "Go to definition") event and executes the function [`bdr.disallow_alter_table_rewrite()`](command:_github.copilot.openSymbolFromReferences?%5B%22bdr.disallow_alter_table_rewrite()%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22external%22%3A%22file%3A%2F%2F%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22path%22%3A%22%2FUsers%2Fhuyantian%2Fgo%2Fsrc%2Fgithub.com%2FEnterpriseDB%2Fbdr%2Fbdr--5.6.0.sql%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3659%2C%22character%22%3A17%7D%7D%5D%5D "Go to definition"). This is typically used to prevent or handle `ALTER TABLE` operations that would cause a table rewrite, ensuring that such operations are either disallowed or managed according to the logic defined in the function.

reference:

https://www.postgresql.eu/events/pgconfeu2019/sessions/session/2641/slides/265/Implementing%20your%20first%20PostgreSQL%20extension.pdf

https://www.postgresql.org/docs/current/extend.html
