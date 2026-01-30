pub trait Message {
    type Response;
}

// struct GetAsset<T> { id: AssetId }
// impl Message for GetAsset<T> {
//     type Response = Handle<T>;
// }
//
// struct PlaySound { id: SoundId }
// impl Message for PlaySound {
//     type Response = ();
// }
