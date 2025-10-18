What is `mod`?  
    A container used to organize and manage code.

How to declare it?  
    1. `pub mod module_name`  
    2. A standalone file is itself a module.

How to import module B inside module A?  
      `mod submodule_example`

How to use methods inside a module?  
    Method 1: Call them as `module::method`  
       ```
       mod submodule_example;
       submodule_example::demonstrate_submodules();
       ```  
    Method 2: Use `use` to bring the desired items into scope.

What is a submodule?  
     You can declare submodules inside a module.

`mod` practice  
1. How to declare it  
2. How to import it  
3. How to create a submodule  
4. How to call a submodule