use crate::compilation::CompilationUnit;

const _: fn() = || {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<CompilationUnit>();
};
