---
id: i4d0v
title: Entrypoint
file_version: 1.1.2
app_version: 1.3.7
---

## What is it?

The entrypoint of the application is`📄 src/main.rs`. It manages scene loading and setup and parallelisation.

### Flow

<br/>

<br/>


<!-- NOTE-swimm-snippet: the lines below link your snippet to Swimm -->
### 📄 src/main.rs
```renderscript
161    fn main() {
```

<br/>


<!-- NOTE-swimm-snippet: the lines below link your snippet to Swimm -->
### 📄 src/main.rs
```renderscript
160    
161    fn main() {
162        //println!("{:?}", helpers::mat_transformation([0.0, 0.0, 0.0], [90.0, 0.0, 0.0], [2.0, 3.0, 4.0]));
163        //return;
164    
165        if configuration::render_scene {
166            render_scene();
167        } else {
168            render_code();
169        }
170    }
171    
```

<br/>

This file was generated by Swimm. [Click here to view it in the app](https://app.swimm.io/repos/Z2l0aHViJTNBJTNBUnVzdFJheU1hcmNoaW5nJTNBJTNBSmVyZW15RnVuaw==/docs/i4d0v).