Automatically obtain all traditional radiomics and deep learning radiomics for Windows 10/11

自动提取传统影像组学和深度学习组学特征

### 当前科研痛点

>科研离不开 python，但是需要的 python 环境却难以搭建，尤其 radiomics、pytorch 等等，不是装不上就是冲突不断，甚至影响整个操作系统环境

与高峰共同研究，尝试在 windows 下安装和配置 python 科研环境，发现 radiomics 和 pytorch 跟很多常用的库，numpy、pandas 以及 matplotlib 等，在自身版本、其它库版本甚至 python 版本间都存在各式各样的冲突，稍不留神就会把环境搞砸，网上教程又鱼龙混杂，不是一键三联就是让你买课，亟需一种方便有效的手段或者方案，让大家从繁琐的环境配置中解脱出来，在自己的轻薄本甚至老笔记本电脑上也能轻轻松松的跑深度学习组学特征提取！

经过阅读相关 dlr 文献，在现有传统影像组学特征提取代码的基础上扩展，编写了全自动配置环境及组学特征提取程序，源码已经公开在 github 上，对此感兴趣的人可以随时签出对源码进行审查和修改。

### 目录和文件名要求、操作流程

一、利用 3D Slicer 对感兴趣取进行分割和勾画，具体勾画流程请参考 3D Slicer 官网，最终将感兴趣的提取组学特征的主序列命名为 main.nrrd，将分割的 ROI 命名为 roi.nrrd，保存在病例目录下，如case1、case2 等等，以此类推，每一个 case 目录下都包含 main.nrrd 和 roi.nrrd。

二、程序项目所在目录下有程序可执行文件、虚拟环境压缩包及 data 目录，如图1。data 目录下有若干 case 目录，如图2。每一个 case 目录下按照前述都存在 main.nrrd 和 roi.nrrd 两个文件，如图3。

三、在程序项目所在目录，双击执行  get_all_radiomics.exe，程序会自动创建虚拟环境，如图4，并自动提取每一个病例的传统组学特征和深度学习组学特征，特征数据分别保存在各自病例目录下，传统组学特征包括约900个特征，深度学习组学特征通过 ResNet 18 激活层获取 1024 维的特征，如有更高维的需求，可以选择 ResNet50、DenseNet121 以及 Transformer 等模型获取特征。

图1. 目录结构如下：
<img width="346" height="129" alt="Pasted image 20251202194345" src="https://github.com/user-attachments/assets/3e725f33-89a3-4d20-89a6-ca1981b0cbf2" />

![[Pasted image 20251202194345.png]]

图2. data 目录下的文件结构：
![[Pasted image 20251202194459.png]]

图3. case 目录下的文件结构：
![[Pasted image 20251202194533.png]]

图4. 自动创建虚拟环境：
![[Pasted image 20251202195407.png]]

图5. 自动提取传统影像组学特征和深度学习组学特征，并保存在各自病例目录下：
![[Pasted image 20251202200904.png]]
![[Pasted image 20251202201340.png]]
![[Pasted image 20251202200954.png]]

### 自动程序带来的好处
1、该自动程序仅在用户目录下创建虚拟环境，不会对系统有任何更改
2、程序会全自动配置环境，避免了 python 或者 anaconda 安装及配置的痛苦
3、按照前述对病例数据目录和文件名的要求，程序会全自动提取全部传统影像组学特征和深度学习组学特征，并自动将特征数据保存在各自病例目录下，全程无需人为干预！

Github: https://github.com/douruixin/get_all_radiomics
已编译的程序及虚拟环境由于体积较大，请在 github 上留言或发邮件索取！

由于是在 linux 下编译的 windows 程序，遇到问题请联系: douruixin@foxmail.com
