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
        update(msg: .event(.get))
        update(msg: .event(.getPlatform))
    }

    private func httpGet(uuid: [UInt8], url: String) {
        Task {
            let (data, _) = try! await URLSession.shared.data(from: URL(string: url)!)
            self.update(msg: .response(uuid, .http(HttpResponse(status: 200, body: [UInt8](data)))))
        }
    }

    func update(msg: Message) {
        let reqs: [Request]

        switch msg {
        case let .event(m):
            reqs = try! [Request].bcsDeserialize(input: iOS.processEvent(try! m.bcsSerialize()))
        case let .response(uuid, outcome):
            reqs = try! [Request].bcsDeserialize(input: iOS.handleResponse(uuid, { switch outcome {
            case let .platform(x):
                return try! x.bcsSerialize()
            case let .time(x):
                return try! x.bcsSerialize()
            case let .http(x):
                return try! x.bcsSerialize()
            case let .key_value(x):
                return try! x.bcsSerialize()
            }}()))
        }

        for req in reqs {
            switch req.effect {
            case .render(_): view = try! ViewModel.bcsDeserialize(input: iOS.view())
            case let .http(r): httpGet(uuid: req.uuid, url: r.url)
            case .time(_):
                update(msg: .response(req.uuid, .time(TimeResponse(value: Date().ISO8601Format()))))
            case .platform(_):
                update(msg: .response(req.uuid, .platform(PlatformResponse(value: get_platform()))))
            case .keyValue(.read):
                update(msg: .response(req.uuid, .key_value(KeyValueOutpu