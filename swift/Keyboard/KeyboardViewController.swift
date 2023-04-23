import SwiftUI
import UIKit

class KeyboardViewController: UIInputViewController {
    override func viewDidLoad() {
        super.viewDidLoad()

        let vc = UIHostingController(rootView: KeyboardWrapper())
        vc.view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        vc.view.translatesAutoresizingMaskIntoConstraints = false

        self.inputView?.addSubview(vc.view)
        self.inputView?.backgroundColor = .clear
    }
}

struct KeyboardWrapper: View {
    @State var counter = 0
    let rowHeight: CGFloat = 54

    
    var body: some View {
        let totalHeight = self.rowHeight * 4
        
        let colors = KhiinColors(KhiinColorScheme.light)
        
        VStack {
            Spacer()
            VStack (spacing: 0) {
                CandidateBarView()
                    .frame(
                        width: .infinity,
                        height: rowHeight)
                KeyboardView(rowHeight)
            }
            .padding(.all, 0)
            .frame(width: .infinity, height: totalHeight)
            .background(colors.backgroundColor)
            .edgesIgnoringSafeArea(.all)
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

struct Key: Hashable {
    let label: String
    let widthPct: CGFloat
    
    init(_ label: String, _ widthPct: CGFloat = 10) {
        self.label = label
        self.widthPct = widthPct
    }
}

struct KeyView: View {
    let key: Key
    let width: CGFloat
    let height: CGFloat
    
    var body: some View {
        let hPad: CGFloat = 4
        let vPad: CGFloat = 12
        Button(action: {}) {
            Text(key.label)
                .font(.system(size: 20, weight: .regular, design: .default))
                .padding(2)
                .frame(width: self.width - hPad, height: self.height - vPad)
                .background(Color.white)
                .cornerRadius(10)
                .foregroundColor(Color.black)
        }.frame(width: self.width, height: self.height)
    }
}
