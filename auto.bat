@echo off  

chcp 65001  
  
title GIT一键提交  
color 0D  
echo 当前目录是：%cd%  
echo;  

echo 请先执行文件init，完成docsify serve docs
echo; 

echo see.查看命令
echo f. 快速提交  
echo 1. 添加变更:git add .  
echo 2. 提交变更  
echo 3. 推送到远程主分支  
echo 4. 切换到主分支  
echo 5. 拉取远程主分支  
echo 6. 主分支合并分支  
echo 7. 拉取远程主分支到本地分支  
echo q. 取消代理
echo X. 退出

echo; 

:continue
set /p choice=请输入数字选择操作:  
echo; 

if "%choice%"=="see" (  
echo f. 快速提交  
echo 1. 添加变更:git add .  
echo 2. 提交变更  
echo 3. 推送到远程主分支  
echo 4. 切换到主分支  
echo 5. 拉取远程主分支  
echo 6. 主分支合并分支  
echo 7. 切换到分支  
echo 8. 拉取远程主分支到本地分支  
echo q. 一键取消代理
echo X. 退出  
echo;  
    goto :continue  
)  

if "%choice%"=="f" (  
    echo 开始添加变更:git add .  
    git add .  
    git commit -m "bat批处理自动推送:%date:~0,10%,%time:~0,8%"  
    git push -u origin main
    echo;  
    goto :continue  
)  

if "%choice%"=="1" (
    echo 开始添加变更:git add .
    git add .
    echo;
    goto :continue
)

if "%choice%"=="2" (
    echo 提交变更:git commit -m "bat批处理自动推送:%date:~0,10%,%time:~0,8%"  
    git commit -m "%declation%"
    echo;
    goto :continue
)

if "%choice%"=="3" (
    echo 将变更情况推送到远程自己分支:git push -u origin main
    git push -u origin main
    echo;
    goto :continue
)

if "%choice%"=="4" (
    echo 切换到主分支:git checkout main
    git checkout main
    echo;
    goto :continue
)

if "%choice%"=="5" (
    echo 本地主分支拉取远程主分支:git pull origin main
    git pull origin main
    echo;
    goto :continue
)

if "%choice%"=="6" (
    
    set /p remoteBranch=请输入远程分支:
    echo 主分支合并分支:git merge %remoteBranch%
    git merge %remoteBranch%
    echo;
    goto :continue
)

if "%choice%"=="7" (  
    echo 请输入要拉取的远程分支和本地分支名称:  
    set /p remoteBranch=请输入远程分支:   
    set /p localBranch=请输入本地分支:   
    echo 本地分支拉取远程主分支:%localBranch%:git pull origin %remoteBranch%  
    git pull origin %remoteBranch%  
    goto :continue  
)  

if "%choice%"=="q" (  
    echo  正在取消代理！
    git config --global --unset http.proxy
    git config --global --unset https.proxy
    echo 执行完毕！
    goto :continue  
)  

if "%choice%"=="X" (
    echo 退出!
    goto :eof
)

:eof
echo 按任意键自动退出！
echo;

pause