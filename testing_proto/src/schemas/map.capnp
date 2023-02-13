@0x84a8d308fefe7cc9;

enum Type {
    a @0;
    b @1;
    c @2;
}

struct Key {
    id @0 :UInt64;
    label @1 :Text;
}

struct Value {
    data @0 :Data;
    type @1 :Type;
}

struct Time {
    minute @0 :UInt64;
    second @1 :UInt64;
}

struct Info {
    time @0 :Time;
}

struct Entry {
    key @0 :Key;
    val @1 :Value;
    info @2 :Info;
}

interface Map {
    get @0 (key :Key) -> (val :Value);
    put @1 (key :Key, val :Value) -> (entry :Entry);
    del @2 (key :Key) -> ();
}
