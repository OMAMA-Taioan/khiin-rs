import SwiftUI

struct KeyboardWrapperView: View {
    @StateObject private var viewModel: KeyboardViewModel
    let rowHeight: CGFloat = 54
    
    init(_ viewModel: KeyboardViewModel) {
        _viewModel = StateObject(
            wrappedValue: viewModel
        )
    }
    
    var body: some View {
        let totalHeight = self.rowHeight * 4
        
        let colors = KhiinColors(KhiinColorScheme.light)
        
        GeometryReader { geometry in
            VStack {
                Spacer()
                VStack (spacing: 0) {
                    CandidateBarView()
                        .frame(
                            width: geometry.size.width,
                            height: rowHeight)
                    KeyboardView(rowHeight)
                }
                .padding(.all, 0)
                .frame(width: geometry.size.width, height: totalHeight)
                .background(colors.backgroundColor)
                .onAppear {
                    print("Hello worlds")
                }
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
    
    init(_ rowHeight: CGFloat) {
        self.rowHeight = rowHeight
    }
    
    var body: some View {
        GeometryReader { geometry in
            VStack(spacing: 0) {
                ForEach(rows, id: \.self) { row in
                    HStack (spacing: 0) {
                        ForEach(row, id: \.self) { key in
                            let width = geometry.size.width / CGFloat(key.widthPct)
                            KeyView(key: key, width: width, height: self.rowHeight)
                        }
                    }
                }
            }
        }
    }
}

enum KeyType {
    case char
    case shift
    case space
    case backspace
    case enter
}

struct Key: Hashable {
    let label: String
    let widthPct: CGFloat
    
    init(_ label: String, _ widthPct: CGFloat = 10) {
        self.label = label
        self.widthPct = widthPct
    }
}

struct KeyView: View {
    @EnvironmentObject var viewModel: KeyboardViewModel
    let key: Key
    let width: CGFloat
    let height: CGFloat
    
    var body: some View {
        let hPad: CGFloat = 4
        let vPad: CGFloat = 12
        Button(action: {
            viewModel.handleKey(self.key)
        }) {
            Text(key.label)
                .font(.system(size: 20, weight: .regular, design: .default))
                .padding(2)
                .frame(width: self.width - hPad, height: self.height - vPad)
                .background(Color.white)
                .cornerRadius(10)
                .foregroundColor(Color.black)
                .shadow(color: Color.gray, radius: 1)
        }.frame(width: self.width, height: self.height)
    }
}
