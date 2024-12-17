### 群峦的锻造公式命令行计算器 Command-line calculator for  TerraFirma

#### build or release

##### 下载 Download

github-release 目前只为win-x86 进行了构建

##### 从源码构建 Build From Source Code

使用 cargo 1.81.0 (2dbb1af80 2024-08-20) 或以上版本进行构建

````
cargo build -r
````

#### usage

参数列表：

````
Usage: TerraFirmaDingDang.exe [OPTIONS]

Options:
  -s, --save-as <SAVE_AS>
          若提供这个参数，则以该参数为key将结构保存到结构文件中
  -c, --load-config <LOAD_CONFIG>
          若提供这个参数，则从文件中以这个key读取结构，同时忽略后面给出的具体参数
  -g, --global
          若提供这个参数，则从用户目录中读取结构文件，而不是可执行文件目录
  -m, --multi-key
          若提供这个参数，指读取key指向的所有结构，而不是这个key的首个结果
  -p, --pipe
          若提供这个参数，则进入管道模式，不输出提示性文字，而是与输入序列相同的格式的字符串
                  输出结果按照以下格式：
                  若数据来源为实时计算，则输出两行，第一行是挽救对齐工件的步骤，第二行是从头开始的步骤：
                  若数据来源为从保存的文件中读取，则输出1或n行，每行均为匹配到key的从头开始的步骤

  -a, --alignment-step <ALIGNMENT_STEP>
          参数内容为：将锻造指针由初始值打造为对齐的步骤
              例子：TFUU 为从初始值进行 轻击-镦锻-收缩-收缩
              填写对照表：
              轻击 (Tapping): T         击打 (Hammering): H
              重击 (HeavyHammering): X  牵拉 (Drawing): D
              冲压 (Stamping): S        弯曲 (Bending): B
              镦锻 (Forging): F         收缩 (Upsetting): U

  -l, --last-steps <LAST_STEPS>
          参数内容为：砧给出的最后步骤的格子，按照从左到右计算
              例子：
              LTSFTU  为三个格子分别为 最后一步轻击-倒数第二部镦锻-倒数第三步收缩
              若第三个格子为空 则填写为 LTSFZ
              填写对照表：
              最后一步为X (Last): L         倒数第二步为X (LastSecond): S
              倒数第三步为X (LastThird): T
              非最后步骤为X (NotLast): N    任意步骤为X (Any): A
              空 (None): Z

  -h, --help
          Print help

````
