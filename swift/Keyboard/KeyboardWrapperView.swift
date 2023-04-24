import SwiftUI

struct KeyboardWrapperView: View {
    let controller: KeyboardViewController
    let rowHeight: CGFloat = 54
    let width: CGFloat
    
    init(
        controller: KeyboardViewController,
        width: CGFloat
    ) {
        self.controller = controller
        self.width = width
    }
    
    var body: some View {
        let colors = KhiinColors(KhiinColorScheme.light)
        
        VStack {
            VStack (spacing: 0) {
                CandidateBarView()
                    .frame(width: self.width, height: self.rowHeight)
                    .onAppear {
                        print("CandidateBarView appeared")
                    }
                KeyboardView(
                    controller: self.controller,
                    width: self.width,
                    rowHeight: self.rowHeight
                )
                .background(colors.backgroundColor)
                .onAppear {
                    print("KeyboardView appeared")
                }
            }
            .padding(.all, 0)
            .background(colors.backgroundColor)
            .onAppear {
                print("Hello world")
            }
        }
    }
}

struct CandidateBarView: View {
    var body: some View {
        HStack {
            Text("起")
            Text("引")
        }
    }
}

struct KeyboardView: View {
    let controller: KeyboardViewController
    let width: CGFloat
    let rowHeight: CGFloat
    
    let rows: [[Key]] = [
        [
            Key("q"),
            Key("w"),
            Key("e"),
            Key("r"),
            Key("t"),
            Key("y"),
            Key("u"),
            Key("i"),
            Key("o"),
            Key("p")
        ],
        [
            Key("a"),
            Key("s"),
            Key("d"),
            Key("f"),
            Key("g"),
            Key("h"),
            Key("j"),
            Key("k"),
            Key("l")
        ],
        [
            Key("z"),
            Key("x"),
            Key("c"),
            Key("v"),
            Key("b"),
            Key("n"),
            Key("m")
        ]
    ]
    
    init(
        controller: KeyboardViewController,
        width: CGFloat,
        rowHeight: CGFloat
    ) {
        self.controller = controller
        self.width = width
        self.rowHeight = rowHeight
    }
    
    func onClick(_ key: Key) -> Void {
        self.controller.handleKey(key: key)
    }
    
    var body: some View {
        VStack(spacing: 0) {
            ForEach(rows, id: \.self) { row in
                HStack (spacing: 0) {
                    ForEach(row, id: \.self) { key in
                        KeyView(
                            key: key,
                            width: self.width / CGFloat(key.widthPct),
                            height: self.rowHeight,
                            onClick: self.onClick
                        )
                    }
                }
            }
        }
    }
}

enum KeyAction: Hashable, Equatable {
    case noop
    case char(Int32)
    case shift
    case space
    case backspace
    case enter
}

struct Key: Hashable, Equatable {
    let label: String
    let widthPct: CGFloat
    let action: KeyAction
    
    init(_ label: String, _ widthPct: CGFloat = 10, _ action: KeyAction = .noop) {
        self.label = label
        self.widthPct = widthPct
        if let firstChar = label.unicodeScalars.first {
            self.action = .char(Int32(firstChar.value))
        } else {
            self.action = action
        }
    }
}

struct KeyView: View {
    let key: Key
    let width: CGFloat
    let height: CGFloat
    let onClick: (Key) -> Void
    
    init(
        key: Key,
        width: CGFloat,
        height: CGFloat,
        onClick: @escaping (Key) -> Void
    ) {
        self.key = key
        self.width = width
        self.height = height
        self.onClick = onClick
    }
    
    var body: some View {
        let hPad: CGFloat = 4
        let vPad: CGFloat = 12
        Button(action: {
            self.onClick(self.key)
        }) {
            Text(key.label)
                .font(.system(size: 20, weight: .regular, design: .default))
                .frame(width: self.width - hPad, height: self.height - vPad)
                .background(Color.white)
                .cornerRadius(10)
                .foregroundColor(Color.black)
                .shadow(color: Color.gray, radius: 1)
        }.frame(width: self.width, height: self.height)
    }
}
