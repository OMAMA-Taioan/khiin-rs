import SwiftUI

enum KhiinColorScheme {
    case dark
    case light
}

struct KhiinColors {
    let backgroundColor: Color
    
    init(_ colorScheme: KhiinColorScheme) {
        switch colorScheme {
        case .light:
            self.backgroundColor = Color(red: 210/255, green: 213/255, blue: 219/255)
        case .dark:
            self.backgroundColor = Color(white:89/255)
        }
    }
}
