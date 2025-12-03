use std::env;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use flate2::read::GzDecoder;
use tar::Archive;
use indicatif::{ProgressBar, ProgressStyle};

fn extract_tar_gz(tar_gz_path: &Path, output_dir: &Path) -> io::Result<()> {
    // 检查输出目录是否已存在
    if output_dir.exists() {
        println!("虚拟环境已存在，跳过创建");
        return Ok(());
    }
    
    // 打开压缩文件
    let tar_gz_file = File::open(tar_gz_path)?;
    let file_size = tar_gz_file.metadata()?.len();
    
    // 创建进度条
    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
        .unwrap()
        .progress_chars("#>-"));
    
    // 创建带进度跟踪的 reader
    let tracked_reader = pb.wrap_read(tar_gz_file);
    
    // gzip 解压
    let tar = GzDecoder::new(tracked_reader);
    // 读取 tar
    let mut archive = Archive::new(tar);

    // 展开到指定目录
    archive.unpack(output_dir)?;
    
    // 完成进度条
    pb.finish_with_message("创建完成");

    Ok(())
}

fn default_output_dir() -> io::Result<PathBuf> {
    if cfg!(target_os = "windows") {
        if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
            return Ok(PathBuf::from(local_app_data).join("radiomics"));
        }
    }
    // 回退到当前工作目录，避免需要管理员权限的路径
    Ok(env::current_dir()?.join("radiomics"))
}

fn main() -> io::Result<()> {
    let tar_path = Path::new("radiomics.env");
    let output_dir = default_output_dir()?;

    // 记录用户当前工作目录，用于解析 data 目录
    let user_cwd = env::current_dir()?;
    // 从可执行文件参数获取 data 路径，默认使用“当前目录/data”
    let data_arg = env::args().nth(1)
        .map(PathBuf::from)
        .map(|p| if p.is_absolute() { p } else { user_cwd.join(p) })
        .unwrap_or_else(|| user_cwd.join("data"));

    // println!("目标环境目录: {}", output_dir.display());
    println!("正在创建虚拟环境...");
    extract_tar_gz(tar_path, &output_dir)?;
    
    // 删除解密后的tar.gz文件
    // let _ = std::fs::remove_file(tar_path);
    
    println!("创建完成!");
    
    // 执行Python脚本
    println!("正在执行处理...");
    let python_exe = output_dir.join("python");
    let script_path = output_dir.join("get_all_features.py");

    let status = Command::new(&python_exe)
        .env("KMP_DUPLICATE_LIB_OK", "TRUE")
        .env("OMP_NUM_THREADS", "1")
        .arg(&script_path)
        .arg(&data_arg)
        // 让 Python 在虚拟环境目录下运行，但 data 参数已提前解析为用户目录下的绝对路径
        .current_dir(&output_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) => {
            // println!("Python返回码: {}", status);
            if !status.success() {
                eprintln!("执行失败！");
            } else {
                println!("");
                println!("传统组学特征和深度学习组学特征全部提取成功！");
                println!("");
                println!("遇到任何问题请联系：douruixin@foxmail.com");
            }
        }
        Err(e) => {
            eprintln!("无法执行操作: {}", e);
        }
    }

    Ok(())
}
