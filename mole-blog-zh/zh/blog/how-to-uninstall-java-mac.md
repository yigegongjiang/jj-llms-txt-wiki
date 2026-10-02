# 从 Mac 卸掉 Java，别弄断工具链

> 先别动系统自带的命令入口，删 JDK 前先盘点已安装版本，再确认还有哪些软件依赖这份运行环境

Published: 2026-08-08 | Updated: 2026-09-05

「在 Mac 上卸载 Java」里藏着两个问题，答错就会把工具链弄坏，第一个是机器上到底有哪一份 Java，第二个是还有哪些软件默认它在。

Oracle 自己的说明很短，最重要的一句不是步骤，而是一条警告：有一个目录不要动。

开始删除支持文件之前，先对照 [Oracle JDK 的现行官方卸载说明](https://docs.oracle.com/en/java/javase/25/install/installation-jdk-macos.html)。如果厂商提供自己的卸载器或应用内命令，应先走那条路径，再处理明确属于它的残留。

这份 Oracle 文档只覆盖通过 Oracle `.pkg` 安装的 JDK，不代表 Temurin、Homebrew 或应用自带运行时。

## 不要从 /usr/bin 里删 Java

Oracle 写得很直接，不要试图通过删除 `/usr/bin` 里的 Java 工具来卸载，该目录属于系统软件，下次 macOS 更新时 Apple 会把改动重置回去。

这是最常见的翻车方式，敲 `which java` 会返回 `/usr/bin` 下的路径，看起来像答案，其实不是，在 macOS 上这些只是指向实际已安装运行环境的占位入口，删掉它们等于拆路标而不是拆楼，macOS 之后还会把它们装回来。

## 先弄清要删的是哪一份 Java

运行环境和开发工具包是不同的安装，卸载方式也不同。

面向普通用户的 Oracle JRE（带浏览器插件和偏好设置面板的运行环境）在 Oracle 点名的三个位置：

```
/Library/Internet Plug-Ins/JavaAppletPlugin.plugin
/Library/PreferencePanes/JavaControlPanel.prefPane
~/Library/Application Support/Oracle/Java
```

JDK 是另一套，装在 `/Library/Java/JavaVirtualMachines/` 下，删除需要管理员权限，可以并排装多个，这很常见，往往是有意为之。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/java-runtimes-and-stubs.webp" width="1360" height="454" loading="lazy" alt="系统目录里的命令行占位入口指向装在 Library Java Virtual Machines 下的 JDK；Oracle 运行环境的插件和偏好设置面板则是另一套独立安装。">
  <figcaption>系统目录里的命令只是指针，不是运行环境本身。删 JDK 是删真实安装；只删指针只会弄断路标，macOS 还会把它们补回来。</figcaption>
</figure>

动手前先列出实际装了什么：

```
/usr/libexec/java_home -V
```

它会打印系统已知的 JDK，包含版本和路径，无论列表有几项，都要先确认常用软件是否依赖准备删除的版本。

## 删错 JDK 会弄坏别的软件

Java 很少为了自己而装，Android 工具链、Elasticsearch、Minecraft 启动器、JetBrains 系列 IDE、Apache 工具，以及部分企业 VPN 和打印客户端，都会自带或依赖 Java 虚拟机，有些内嵌了自己的副本，不受影响，有些解析系统提供的那一份，环境一变就停工。

若 JDK 是用包管理器装的，就用同样方式卸，Homebrew 装的 temurin 用 `brew uninstall` 卸掉，手工抠文件会让包数据库仍认为它在。

## 用官方支持的方式删 JDK

若 JDK 是用 `.pkg` 装的，删除对象是 `/Library/Java/JavaVirtualMachines/` 下带版本号的那一层目录，并需要 `sudo`，一次只删一个版本，确认常用工具仍正常，再删下一个。

之后 `/usr/libexec/java_home -V` 不应再列出它，若曾在 shell 配置里手写 `JAVA_HOME`，它现在会指向空处，这个环境变量是「明明卸干净了工具链却挂了」的第二常见原因。

## 关掉终端前先核对

```
/usr/libexec/java_home -V
java -version
```

第一条显示还剩什么，第二条要么报出版本，要么提示没有已安装的运行环境，在你本意就是清空的机器上，后一种才是正确结果，不是报错。

## 整块占用视图能帮上什么

Java 的相关文件分散在缓存、偏好设置、插件和设置面板等位置，先看终端列出的运行环境，再确认各项归属。如果还想查看某个 JDK 或内置 JDK 的应用占了多少空间，可以用 [Mole](https://mole.fit/zh/) 扫描后展开对应目录。

## 删完之后要回头看的三处

第一处是 shell 配置，`~/.zshrc` 和 `~/.zprofile` 里手写的 `JAVA_HOME` 不会随 JDK 一起消失，它会继续指向一个已经不存在的路径，之后每个新 shell 都带着这个坏变量，删掉那一行，或者改成 `export JAVA_HOME=$(/usr/libexec/java_home)`，让它跟着系统走。

第二处是 IDE，JetBrains 系和 Android Studio 会在项目设置里记住具体的 JDK 路径，系统里换了版本它们不会自己跟着改，表现是项目突然编不过而命令行一切正常。

第三处是包管理器装的那些，`brew list | grep -i -E 'jdk|temurin|zulu|openjdk'` 能看出哪些是 Homebrew 装的，它们要用 `brew uninstall` 卸，手工删掉目录会让 Homebrew 的数据库继续以为它还在。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-java-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
