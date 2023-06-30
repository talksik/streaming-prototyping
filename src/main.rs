use livekit::prelude::*;

// Connect to a room using the specified env variables
// and print all incoming events

#[tokio::main]
async fn main() {
    // run livekit-server locally: livekit-server --dev
    let url = "ws://localhost:7880";
    // run livekit-cli command:
        // livekit-cli create-token \
        // --api-key devkey --api-secret secret \
        // --join --room my-first-room --identity user1 \
        // --valid-for 24h
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2ODgyNDk3NzEsImlzcyI6ImRldmtleSIsIm5hbWUiOiJ1c2VyMSIsIm5iZiI6MTY4ODE2MzM3MSwic3ViIjoidXNlcjEiLCJ2aWRlbyI6eyJyb29tIjoibXktZmlyc3Qtcm9vbSIsInJvb21Kb2luIjp0cnVlfX0.HLDtRZbgca9UTVjV0d8Ms1ukv_eCH11AoCqyN_fyt88";

    let (room, mut rx) = Room::connect(url, token, RoomOptions::default()).await.unwrap();
    println!("Connected to room: {} - {}", room.name(), room.sid());

    while let Some(msg) = rx.recv().await {
        println!("Event: {:?}", msg);
    }
}
