
# Issue

```
error[E0554]: `#![feature]` may not be used on the stable release channel
```


# Fix 


> 从错误信息`#![feature]` may not be used on the stable release channel可以看出当前编译使用的channel还没有包含#![feature]功能，那咋办呢?换channel呗!换之前是不是要先了解下Channel指的是啥？都有哪些可用的Channel? 简单来说channel就是代表我们使用的Rust开发环境是稳定版的，还是试用版的，还是尝鲜版的？就像我们平常开发软件一样一样的，在软件里添加了新功能不能直接把线上的稳定版软件替换掉，因为新版本可能还有bug需要试用一段，试用一段确认没问题了再替换原来的稳定版。稳定版试用版尝鲜版分别对应stable,beta,nightly.在stable里没有的功能，可能在beta和nightly里就有了。要使用beta和nightly版首先要看下有没有安装:
> 
> $ rustup toolchain list
> stable-x86_64-unknown-linux-gnu (default)
> 可以看当当前环境只安装了稳定(stable)版,接下来其它Channel,以安装nightly为例：
> 
> $ rustup toolchain install nightly
> 安装的时候也可以指定具体的版本信息，默认安装最新的.
> 
> 安装好后怎么使用呢？
> 
> 方式一：比较简单的方式是直接安装加更改当前系统默认的channel
> 
>  $ rustup default nightly
> 这种方式连上面的安装步骤都一起做了，直接再执行cargo build 都是使用的nightly channel编译构建项目了，也就是原来使用stable的项目现在也改成nightly了，可能我们并不想都改,咋办?能不是只是临时的用下，可以的:
> 
> 方式二：使用rustup run指定channel
> 
> $ rustup run nightly cargo build
> 要是就临时这么写一次也没什么，如果用的多了cargo build前面总是要多敲一串难免麻烦，能不能在当前项目中就默认是nightly，不影响其它项目呢?也是可以的.
> 
> 方式三: 使用rustup overwrite设置当前项目使用的channel 
> 
> 进入项目目录执行:
> 
> $ rustup override set nightly
> 再执行cargo build就不报错了.
> 
> 问题解决

# Reference 

https://www.cnblogs.com/pu369/p/15161194.html


# Concepts

todo... 
