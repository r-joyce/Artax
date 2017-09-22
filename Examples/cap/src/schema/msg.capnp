@0x864eb5bba58086d8; 

struct TimeStampMessage {
  timeStamp @0 :List(UInt64);
}

struct TicMessage {
  ticVec @0 :List(UInt32);
}

struct MzMessage {
  mzVec @0 :List(UInt16);
}
