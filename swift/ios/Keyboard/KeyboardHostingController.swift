import SwiftUI

class KeyboardHostingController<Content: View>: UIHostingController<Content> {
    public func add(to controller: KeyboardViewController) {
        controller.addChild(self)
        controller.view.addSubview(self.view)
        self.didMove(toParent: controller)
        self.view.backgroundColor = .clear
        self.view.translatesAutoresizingMaskIntoConstraints = false
        self.view.leadingAnchor
            .constraint(equalTo: controller.view.leadingAnchor)
            .isActive = true
        self.view.trailingAnchor
            .constraint(equalTo: controller.view.trailingAnchor)
            .isActive = true
        self.view.topAnchor
            .constraint(equalTo: controller.view.topAnchor)
            .isActive = true
        self.view.bottomAnchor
            .constraint(equalTo: controller.view.bottomAnchor)
            .isActive = true
    }
    
    deinit {
        self.removeFromParent()
        self.view.removeFromSuperview()
    }
    
    public override func viewWillLayoutSubviews() {
        super.viewWillLayoutSubviews()
        self.updateViewConstraints()
    }
}
