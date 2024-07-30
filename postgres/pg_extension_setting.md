Writing a PostgreSQL extension that includes a configuration parameter requiring a server restart upon change involves several steps. In PostgreSQL, such parameters are known as "Postmaster" level settings. Here’s a general outline for defining such a setting in your extension:

1. **Define the Custom GUC Variable:**
   GUC (Grand Unified Configuration) system in PostgreSQL allows the definition of custom variables. To define a variable that requires a server restart, you would typically use the `DefineCustomStringVariable`, `DefineCustomIntVariable`, `DefineCustomBoolVariable`, or similar functions depending on the type of your variable. In the definition, you will set the context to `PGC_POSTMASTER` to indicate that changes require a server restart.

   Example:
   ```c
   DefineCustomIntVariable("myextension.my_variable",
                           "Description of what this variable does.",
                           NULL,
                           &my_variable,
                           0,  /* default value */
                           0,  /* minimum value */
                           INT_MAX, /* maximum value */
                           PGC_POSTMASTER,  /* indicates a postmaster level variable */
                           0,
                           NULL,
                           NULL,
                           NULL);
   ```

   In this example, `my_variable` is the C variable where the value of the GUC variable will be stored.

2. **Initialize Your Extension:**
   Your extension needs an initialization function where you will define your GUC variables. This function is typically named `_PG_init` and is automatically called when PostgreSQL loads your extension.

   Example:
   ```c
   void
   _PG_init(void)
   {
       /* Define all GUC variables here */
   }
   ```

3. **Compile Your Extension:**
   After writing your extension, compile it into a shared library. This typically involves writing a `Makefile` and using the PostgreSQL development tools like `pgxs` to compile it.

4. **Loading the Extension:**
   Once compiled, you load the extension into PostgreSQL. This is usually done by adding a line to the `postgresql.conf` file or using the `CREATE EXTENSION` SQL command, depending on how your extension is designed to be loaded.

5. **Changing the Variable:**
   After loading the extension, you can change the variable in `postgresql.conf`. For example:
   ```
   # in postgresql.conf
   myextension.my_variable = 42
   ```
   Since this is a `PGC_POSTMASTER` level setting, changing this variable requires restarting the PostgreSQL server for the new value to take effect.

Remember, developing PostgreSQL extensions requires good knowledge of C programming and understanding PostgreSQL internals. Testing the extension thoroughly is crucial before deploying it in a production environment.
