import SwiftUI

extension KhiinInputController {
    func resetWindow() {
        _ = self.window?.contentView?.subviews.map({ $0.removeFromSuperview() })
        _ = self.window?.contentViewController?.children.map({
            $0.removeFromParent()
        })

        let frame: CGRect = self.windowFrame()

        log.debug("Resetting window to frame: \(frame)")

        if self.window == nil {
            self.window = NSWindow(
                contentRect: frame,
                styleMask: .borderless,
                backing: .buffered,
                defer: false
            )
            self.window?.collectionBehavior = .moveToActiveSpace
            let levelValue: Int = Int(CGShieldingWindowLevel())
            self.window?.level = NSWindow.Level(levelValue)
            self.window?.backgroundColor = .clear
        }

        let candidateView = NSHostingController(
            rootView: CandidateView().environmentObject(self.candidateViewModel)
        )

        self.window?.contentView?.addSubview(candidateView.view)

        candidateView.view.translatesAutoresizingMaskIntoConstraints = false
        
        if let topAnchor = window?.contentView?.topAnchor,
            // let bottomAnchor = window?.contentView?.bottomAnchor,
            let leadingAnchor = window?.contentView?.leadingAnchor
            // let trailingAnchor = window?.contentView?.trailingAnchor
        {
            NSLayoutConstraint.activate([
                candidateView.view.topAnchor.constraint(
                    equalTo: topAnchor
                ),
                candidateView.view.leadingAnchor.constraint(
                    equalTo: leadingAnchor
                ),
            ])
        }
        self.window?.contentViewController?.addChild(candidateView)
        self.window?.setFrame(frame, display: true)
        self.window?.orderFrontRegardless()
    }

    func windowFrame() -> CGRect {
        let height: CGFloat = 24 * 9 + 8 * 2
        let origin = self.currentOrigin ?? self.currentClient?.position ?? .zero
        let size = CGSize(width: 500, height: height)

        return CGRect(
            x: origin.x, y: origin.y - height, width: size.width, height: size.height)
    }
}
