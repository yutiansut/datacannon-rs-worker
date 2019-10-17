/// Traits for connections
/// Author: Andrew Evans


trait Connection{
    fn open(self);
    fn send(self);
    fn close(self);
}
