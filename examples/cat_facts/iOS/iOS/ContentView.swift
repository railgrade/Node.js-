import Serde
import SwiftUI

func get_platform() -> String {
    return UIDevice.current.systemName + " " + UIDevice.current.systemVersion
}

enum Outcome {
    case platform(PlatformResponse)
    case time(TimeResponse)
    case http(HttpResponse)
    case key_value(KeyValueOutput)
}

enum Message {
    case event(Event)
    case response([UInt8], Outcome)
}

@MainActor
class Model: ObservableObject {
    @Published var view = ViewModel(fact: "", image: .none, platform: "")

    init() {
        upda