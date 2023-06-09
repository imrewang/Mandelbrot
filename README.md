# mandelbrot

## 一、简介

使用Rust编写了一个输出Mandelbrot集合图像的项目

## 二、部署

### （一）运行

执行 **cargo new mandelbrot --vcs none**，创建出一个新的空白项目

执行 **cd mandelbrot**，移动到这个新项目的根目录中

执行 **cargo add num**，编辑 Cargo toml，把num 软件包作为依赖项添加进来

执行 **cargo run** 命令。

## 三、总结

**1.cargo.toml的作用**

Cargo.toml是Rust项目的配置文件，它描述了项目的元数据和依赖项。在Rust项目中，Cargo.toml文件是非常重要的，因为它包含了项目的所有信息和配置，包括：

项目的名称、版本、作者、描述等基本信息。

项目的依赖项，包括Rust本身和其他外部库。

编译选项、测试选项和文档选项等配置信息。

发布选项，包括发布到哪个仓库、发布的版本号等。

使用Cargo.toml文件可以使Rust项目的依赖管理和构建变得非常简单，因为Cargo可以根据Cargo.toml文件中的信息自动下载和安装依赖项，并自动构建和打包项目。

**2.cargo.lock的作用**

cargo.lock文件是Rust项目中的一个锁定文件。它记录了项目的依赖项的确切版本号和子依赖项的版本，以及它们所依赖的其他库的版本。

具体来说，cargo.lock文件包含了：

项目依赖项的名称、版本号和依赖类型（build-dependencies、dev-dependencies或dependencies）等信息。

依赖项所依赖的其他库的版本号和子依赖项的版本。

依赖项的URL、checksum和路径等信息，以确保依赖项的安全性和稳定性。

cargo.lock文件的作用是锁定项目的依赖项，**以确保项目在不同的环境中运行时使用的是相同的依赖项版本**。

这可以避免由于依赖项版本的不同而导致项目编译或运行错误的情况。

在使用Cargo构建项目时，Cargo会检查cargo.lock文件以确保依赖项的版本与该文件中的版本匹配。

如果依赖项的版本发生变化，Cargo会自动更新cargo.lock文件，并记录新的版本号。

## 四、改进方向

**1.颜色**

使用图像库，添加颜色，输出为图片


