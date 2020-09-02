# First

install.packages("tidyverse")
library(tidyverse)
Error in library(tidyverse) : 不存在叫‘tidyverse’这个名字的程辑包

Solution was to install a dependency:

install.packages("cellranger")
(answered in comments by @neilfws)


# ## Example ~/.Renviron on Unix
# R_LIBS=~/R/library
# PAGER=/usr/local/bin/less

/usr/lib/R/library"

R (as 'lib' is unspecified)
The answer:

Those are not errors, they are just notes to let you know where it was 
installed.

It is so ...

---

 library("devtools", lib.loc="~/R/library")
  library("bookdown", lib.loc="~/R/library")
  library("bookdownplus", lib.loc="~/R/library")
bookdownplus
devtools::install_github("pzhaonet/bookdownplus")

  ---

#  书籍排版软件 R bookdown 安装
96  空中指南 
 0.7 2018.03.07 13:55* 字数 535 阅读 1299评论 0喜欢 7
简介
R bookdown 可以看作使用 Markdown 进行排版的 Latex 软件。你用 Markdown 来写文章或书籍，它用 Latex 渲染出来，输出排版优美的 pdf 书籍。也可以输出各种 pandoc 支持的电子书格式。

安装方法
1.安装R
到下面地址下载 R for windows 并安装
https://cran.r-project.org/bin/windows/base/

2.安装 R bookdown
在开始菜单运行 R x64，在命令行中依次输入、运行下面两行安装组件

install.packages("devtools")
devtools::install_github("rstudio/bookdown")


这时会要求选择下载镜像地址，可以选择 China Shanghai 。这时会要求选择下载镜像地址，可以选择 China Shanghai 。下载安装可能需要几分钟。

R bookdown 的文档：https://bookdown.org/home/getting-started.html

安装 TinyTex
在开始菜单运行 R x64，在命令行中依次输入、运行下面两行安装组件

devtools::install_github('yihui/tinytex')
tinytex::install_tinytex()


这时会要求选择下载镜像地址，可以选择 China Shanghai 。下载安装可能需要5分钟左右。

TinyTex 的中文文档：https://yihui.name/tinytex/cn/

安装 RStudio IDE
下载地址 https://www.rstudio.com/products/rstudio/download/preview/

这样所需的程序就全部安装完成了

测试运行
下载 Demo 文件
https://github.com/rstudio/bookdown-demo/archive/master.zip
解压缩后双击其中的 bookdown-demo.Rproj 文件。
image.png
渲染 Demo书籍
运行后会看见如下界面，点击 Build Books 按钮则会渲染出 PDF 书籍来


image.png

渲染后的 PDF 文件在源文件的 _book 文件夹下面


image.png
image.png
看看效果，是不是很不错呢？

可以照着刚才下载的例子来学习怎么编写书籍。

或者这里有一个最简单的 Hello World！形式的书籍例子，也可以下载来参考下 https://github.com/yihui/bookdown-minimal.

现在已经有了 bookdown 的加强版， bookdownplus, 建议安装这个
安装方法见：http://www.pzhao.org/zh/post/bookdownplus-released/
2018.11的作者写的升级手记：http://www.pzhao.org/zh/post/-bookdownplus-update/

小礼物走一走，来简书关注我
**bookdown::render_book('index.Rmd', 'all')**

---

1:   ggplot2   (3.1.0 -> 230e8f78a...) [GitHub]
2:   rmarkdown (1.12  -> 63418e258...) [GitHub]
3:   CRAN packages only
4:   All
5:   None

bookdown::pdf_book(toc=TRUE,number_sections = TRUE,fig_caption = NULL,...,base_format = rmarkdown::pdf_document,toc_unnumbered = TRUE,toc_appendix = false,
pdf_book(toc = TRUE, number_sections = TRUE, fig_caption = TRUE, 
  pandoc_args = NULL, ..., base_format = rmarkdown::pdf_document, 
  toc_unnumbered = TRUE, toc_appendix = FALSE, toc_bib = FALSE, 
  quote_footer = NULL, highlight_bw = FALSE)

  ---
  R 包 bookdownplus 正式发布
Thu, Jun 22, 2017  R, bookdown, bookdownplus, post
很高兴地宣布，我的 R 语言扩展包 ‘bookdownplus v1.0.2’ (Zhao 2017a) 在 CRAN 正式发布了。本文是对’bookdownplus v1.0.2’的简要描述. 由于最近更新频繁, 最新版的’bookdownplus’更加强大和易用, 例如 ‘bookdownplus v1.2.0’ 可以一条指令生成19个模板文档的 pdf, word, html 和 epub 四种格式. 建议移步位于 GitHub 的 bookdownplus 项目主页.




简介
bookdownplus 是对 bookdown 包 (Xie 2016) 的增强和简化, 是快速使用 bookdown 的最简洁方法。bookdown 就好比 Microsoft Word 或 
L
A
T
E
X
，可以用来写文档，而 bookdownplus 提供了很多有用的模板，可以很方便地在 bookdown 平台写期刊论文、学位论文、学术海报、化学分子式、信件、日记、日历、诗集、吉他谱等各种常用文档和书籍。这是功能上的增强（+）。

bookdownplus 使用时只需指定一个模板，给定作者和书名，就可以一键生成模板文件，用户在模板文件里照猫画虎写自己的文字就可以了，不必再花力气上网找模板、设置 YAML 和 LaTeX。这是操作上的简化（-）。

bookdownplus 各个模板的使用方法详见 R bookdownplus Textbook。这本电子书本身就是用 bookdownplus 生成的，尤其是它的 pdf 版本很美观。此书的源码开放，可以作为使用 bookdownplus写书的示例。

下面是 bookdownplus 的简介和快速使用方法。

快速入门
准备
在开始前，需要安装 R, RStudio, bookdown，和其他依赖的软件和包(例如 Pandoc, 
L
A
T
E
X
, rmarkdown, rticle, knitr等)，作为准备。详见 bookdown 官方手册。

安装
准备完毕后，就可以安装 bookdownplus 了。可以安装稳定版：

install.packages("bookdownplus")
或开发版：

devtools::install_github("pzhaonet/bookdownplus")
生成模板文件
接着，在 R 中运行下面的代码：

bookdownplus::bookdownplus()
这时，在你的工作目录（getwd()），会得到一些模板文件（如 index.Rmd，body.Rmd， bookdownplus.Rproj） 和文件夹。

编译成书
用 RStudio 打开 bookdownplus.Rproj文件，然后按 ctrl+shift+b，Duang！你就得到模板书 *.pdf了！保存在 _book/ 文件夹里。

你的文字
在 index.Rmd 和 body.Rmd 里写你自己的文字，享受写书的快乐吧！自古皆死，不朽者文。

更多输出格式
模板默认生成的书是pdf格式。‘bookdownplus’ 从 1.0.3 开始，可以很方便地输出更多格式，包括国内最常见的 ’word’格式，网页’html’格式和电子书’epub’格式，只需运行：

bookdownplus::bookdownplus(template = 'article', more_output = c('html', 'word', 'epub'))
就可以在 _book/ 文件夹里看到这些文件了。

网页格式可以极其方便地免费发布到 bookdown.org，只需运行：

bookdown::publish_book()
这里是 bookdown 书籍的大本营。截至 2017 年 6 月，我有三本书被放置在这个网站的首页。

更多建议
我开发的另外两个 R 包可以配合 ‘bookdown’ 使用：

mindr (Zhao 2017b)，可以从 markdown 或 R markdown 格式的书稿中提取纲要，并且生成思维导图。

pinyin (Zhao 2017c)，可以为书稿的章节标题自动生成‘{#ID}’。如果标题里含有汉字，就会自动转换成拼音。

具体用法见他们的帮助信息。这两个包已经在 CRAN正式发布，安装命令是：

install.packages('mindr')
install.packages('pinyin')
更改模板
上述快速入门得到的模板文档是默认的论文文档。只需在上述步骤里为 bookdownplus() 函数指定模板名称，就可以用类似的操作得到其他模板的示范文档，如：

bookdownplus::bookdownplus(template = 'poem')
下面展示一些模板。

article

article_mdpi1

article_zh

calendar

chemistry

guitar

journal

mail


poem

thesis_classic

thesis_ubt

thesis_zju

yihui_demo

yihui_mini

yihui_zh

poster


更新记录
2017-06-21. Released on CRAN!
2017-06-21. Version 1.0.2. Resubmitted to CRAN!
2017-06-21. Version 1.0.2. Resubmitted to CRAN!
2017-06-21. Version 1.0.1. Submitted to CRAN!
2017-06-14. Version 0.0.1. A bug fixed.
2017-05-15. Version 0.0.0. A very preliminary version.
Xie, Yihui. 2016. Bookdown: Authoring Books and Technical Documents with R Markdown. Boca Raton, Florida: Chapman; Hall/CRC. https://github.com/rstudio/bookdown.

Zhao, Peng. 2017a. Bookdownplus: Generate Varied Books and Documents with R ’Bookdown’ Package. https://CRAN.R-project.org/package=bookdownplus.

———. 2017b. Mindr: Convert Files Between Markdown or Rmarkdown Files and Mindmaps. https://github.com/pzhaonet/mindr.

———. 2017c. Pinyin: Convert Chinese Characters into Pinyin. https://github.com/pzhaonet/pinyin.

← LaTeX 公式转换成 Word 公式 R 初学者包 beginr 在 CRAN 正式发布 →
comments powered by Disqus


### bookdownplus
installing 11 packages: blogdown, bookdownplus, data.tree, DiagrammeR, influenceR, mindr, rgexf, Rook, rosr, visNetwork, XML