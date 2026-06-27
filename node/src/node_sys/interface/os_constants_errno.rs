use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type OsConstantsErrno;

    #[wasm_bindgen(method, getter)]
    pub fn E2BIG(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EACCES(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EADDRINUSE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EADDRNOTAVAIL(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EAFNOSUPPORT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EAGAIN(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EALREADY(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EBADF(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EBADMSG(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EBUSY(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ECANCELED(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ECHILD(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ECONNABORTED(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ECONNREFUSED(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ECONNRESET(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EDEADLK(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EDESTADDRREQ(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EDOM(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EDQUOT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EEXIST(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EFAULT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EFBIG(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EHOSTUNREACH(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EIDRM(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EILSEQ(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EINPROGRESS(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EINTR(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EINVAL(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EIO(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EISCONN(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EISDIR(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ELOOP(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EMFILE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EMLINK(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EMSGSIZE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EMULTIHOP(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENAMETOOLONG(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENETDOWN(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENETRESET(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENETUNREACH(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENFILE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOBUFS(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENODATA(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENODEV(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOENT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOEXEC(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOLCK(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOLINK(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOMEM(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOMSG(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOPROTOOPT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOSPC(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOSR(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOSTR(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOSYS(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOTCONN(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOTDIR(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOTEMPTY(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOTSOCK(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOTSUP(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENOTTY(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ENXIO(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EOPNOTSUPP(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EOVERFLOW(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EPERM(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EPIPE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EPROTO(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EPROTONOSUPPORT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EPROTOTYPE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ERANGE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EROFS(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ESPIPE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ESRCH(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ESTALE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ETIME(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ETIMEDOUT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn ETXTBSY(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EWOULDBLOCK(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn EXDEV(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEINTR(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEBADF(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEACCESS(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WASEFAULT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEINVAL(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WASEMFILE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEWOULDBLOCK(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEINPROGRESS(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEALREADY(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAENOTSOCK(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEDESTADDRREQ(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEMSGSIZE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEPROTOTYPE(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAENOPROTOOPT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAEPROTONOSUPPORT(this: &OsConstantsErrno) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn WSAESOCKTNOSUPPORT(this: &OsConstantsErrno) -> i32;
}
