# buddy-system-in-ucore-test

### 功能描述：
为了能够在c语言程序中调用buddy system的接口，对buddy system进行了一定的封装。

目前在buddy_system目录下是使用rust实现的buddy system，而rbuddy_system目录下的代码封装了buddy_system中的内存分配和释放的功能，即在/rbuddy_system/src/lib.rs中中封装好了函数init()、alloc()、free()。

首先划分出了64MB的内存作为堆内存，然后定义了全局的堆分配器（ALLOCATOR）。init()函数主要完成了buddy system分配器的初始化，alloc()函数则对buddy_system的分配函数进行了分装，最终返回一个页面的地址，而free()封装了buddy_system的内存释放函数。

### 编译：
进入rbuddy_system目录，首先将lib.rs编译成静态的库，修改Cargo.toml，设置"crate-type"为“["staticlib"]”，然后使用命令“cargo build --target riscv64gc-unknown-none-elf”进行编译，可以得到“librbuddy_system.a”静态库。

### 测试：
修改ucore项目代码，在main()函数中添加调用init()函数的代码，并将ucore代码中的kalloc()函数替换成alloc()，kfree替换成free()，要在C程序中调用init()、alloc()和free()函数需要先通过extern关键字引入。

将静态库拷贝到ucore项目中，修改Makefile，添加对静态库的编译，然后编译。

### 结果：
ucore正常运行，user中的测例也正常运行。
