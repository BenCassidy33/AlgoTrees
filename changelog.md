# Change Log for AlgoTrees

This change log provides an overview of the changes made to the AlgoTrees project, including potential impact on existing projects.

## Jun 24, 2023

These changes were implemented to better reflect the purpose and implementation of each function. Please note that these are **VERY** breaking changes and should be adopted only after making appropriate adjustments to accommodate these changes.

### Algorithms Module

- Updated `get_left()` ⟶ `get_far_left()` to maintain clarity and reduce conflict between modules. **Breaking Change**
- Updated `get_right()` ⟶ `get_far_right()` to maintain clarity and reduce conflict between modules. **Breaking Change**
- Updated Algorithms Module to include `width()`, `get_far_left()`, `get_far_right()` as dot functions. **Breaking Change**

### Actions Module

- Updated `get_right()` ⟶ `get_right_node()` to better reflect the function's purpose. **Breaking Change**
- Updated `get_left()` ⟶ `get_left_node()` to better reflect the function's purpose. **Breaking Change**
- Updated `get_left_val()` ⟶ `get_left_head_val()` to better reflect the function's purpose and reduce conflict between modules. **Breaking Change**
- Updated `get_right_val()` ⟶ `get_right_head_val()` to better reflect the function's purpose and reduce conflict between modules. **Breaking Change**

