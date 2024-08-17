/// Refer [https://dsa.cs.tsinghua.edu.cn/oj/static/unix_signal.html]
#[derive(Clone, Copy, Debug)]
pub enum SIGNAL {
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGALARM = 14,
    SIGTERM = 15,
    SIGSTOP = 17,
}
