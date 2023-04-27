import InputMethodKit
import SwiftyBeaver

@objc(InputController)
class KhiinInputController: IMKInputController {
    override func activateServer(_ sender: Any!) {
        Logger.setup()
        EngineController.instance.reset()
    }
    
    override func inputText(_ string: String!, client sender: Any!) -> Bool {
        log.debug("inputText: \(string ?? "n/a")")

        guard let client = sender as? IMKTextInput else {
            return false
        }

       if let first = string.first, first.isASCII && first.isLetter {
           let engine = EngineController.instance

           let cmd = engine.handleChar(Int32(first.asciiValue!))
           if let cand = cmd?.response.candidateList.candidates.first?.value {
               client.insertText(
                   cand,
                   replacementRange: NSRange(
                       location: NSNotFound,
                       length: NSNotFound
                   )
               )

               return true
           }
       }

        client.insertText(
            string + string + string,
            replacementRange: NSRange(
                location: NSNotFound,
                length: NSNotFound
            )
        )

        return true
    }
}
