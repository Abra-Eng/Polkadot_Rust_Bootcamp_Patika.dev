# Finding max value of an array of integers

**Created array**
```rust
let numbers: [i32; 7]= [2,5,8,19,7,23,1];
```

**Create the find_max function and set it's parameter to an array then create a mutable variable to store and return the max value and set it's value to the num[0]**

```rust
fn find_max(num: &[i32]) -> i32 {}
let mut max_value = num[0];
```

**Using for loop iterate through the array and with control flow find the maximum value and change the max_value with it. then return the max_value**

```rust
for &number in num {
        if  number > max_value{
            max_value = number;
        }
    }
max_value
```

**Finally get the return value and print it**
```rust
let max_number = find_max(&numbers);
   
println!("The max number in this slice is {}", max_number);
```

