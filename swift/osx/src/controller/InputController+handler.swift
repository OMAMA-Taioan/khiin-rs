import InputMethodKit

extension KhiinInputController {
    override func recognizedEvents(_ sender: Any!) -> Int {
        // flagsChanged is needed to see the shift key on its own, which the
        // user can pick as the input mode shortcut.
        let masks: NSEvent.EventTypeMask = [.keyDown, .flagsChanged]
        return Int(masks.rawValue)
    }

    // Tracks the shift key so that tapping it on its own switches the input
    // mode, the same shortcut the Windows IME offers. It only counts when
    // shift goes down and comes back up with nothing else in between: any
    // other key (handled above) or any other modifier cancels it. The event is
    // never swallowed, so shift keeps working as a normal modifier.
    func handleFlagsChanged(_ event: NSEvent!, client sender: Any!) -> Bool {
        let modifiers = event.modifierFlags

        guard (event.keyCode == KeyCode.Modifier.VK_SHIFT_LEFT
            || event.keyCode == KeyCode.Modifier.VK_SHIFT_RIGHT) else {
            // Another modifier joined in, so shift is modifying something.
            self.shiftHeldAlone = false
            return false
        }

        if (modifiers.contains(.shift)) {
            self.shiftHeldAlone = modifiers
                .intersection([.command, .control, .option])
                .isEmpty
            return false
        }

        // Shift released.
        guard self.shiftHeldAlone else {
            return false
        }
        self.shiftHeldAlone = false

        guard self.isInputModeShortcutShift() else {
            return false
        }

        guard let client: IMKTextInput = sender as? IMKTextInput else {
            return false
        }

        log.debug("toggle input mode by shift key")
        _ = self.commitAll()
        self.candidateViewModel.changeInputMode()
        self.reset()
        client.clearMarkedText()
        return false
    }

    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        if (event.type == .flagsChanged) {
            return self.handleFlagsChanged(event, client: sender)
        }

        // Any key pressed while shift is held means shift was used as a
        // modifier, so it must not switch the input mode when released.
        self.shiftHeldAlone = false

        let modifiers = event.modifierFlags
        // With shift chosen as the shortcut, alt + ` no longer switches the
        // input mode, matching how Windows drops ctrl + ` in that case.
        let changeInputMode = modifiers.contains(.option)
            && event.keyCode.representative == .punctuation("`")
            && !self.isInputModeShortcutShift()
        let shouldIgnoreCurrentEvent: Bool =
            !changeInputMode && (modifiers.contains(.command) || modifiers.contains(.option))
        
        guard let client: IMKTextInput = sender as? IMKTextInput else {
            return false
        }
        currentOrigin = client.position

        log.debug("Current origin: \(String(describing: currentOrigin))")

        // alt + h or alt + s, change to hanji first
        if (modifiers.contains(.option) && (event.keyCode.representative == .alphabet("h") || event.keyCode.representative == .alphabet("s"))) {
            _ = self.commitAll();
            self.candidateViewModel.changeOutputMode(isHanjiFirst: true)
            self.reset()
            client.clearMarkedText()
            return true
        } else if (modifiers.contains(.option) && event.keyCode.representative == .alphabet("l")) {
            _ = self.commitAll();
            self.candidateViewModel.changeOutputMode(isHanjiFirst: false)
            self.reset()
            client.clearMarkedText()
            return true
        } else if (modifiers.contains(.option) && event.keyCode.representative == .space) {
            // alt + space, toggle output mode
            _ = self.commitAll();
            self.candidateViewModel.toggleOutputMode()
            self.reset()
            client.clearMarkedText()
            return true
        } else if (changeInputMode) {
            _ = self.commitAll();
            self.candidateViewModel.changeInputMode()
            self.reset()
            client.clearMarkedText()
            return true
        } else if (shouldIgnoreCurrentEvent) {
            _ = self.commitAll();
            self.candidateViewModel.reset()
            return false;
        }
        if (self.isClassicMode()) {
            if (event.characters == "'") {
                log.debug("handle punctuation '" + event.characters!)
                return self.handlePunctuation("'");
            } else if (event.characters == "\"") {
                log.debug("handle punctuation \"" + event.characters!)
                return self.handlePunctuation("\"");
            } else if (event.characters == ":") {
                log.debug("handle punctuation :" + event.characters!)
                return self.handlePunctuation(":");
            }
        }
        switch event.keyCode.representative {
            case .alphabet(var char):
                if (self.isManualMode()) {
                    if ((self.currentDisplayText().hasSuffix("-") || self.currentDisplayText().hasSuffix("·")) 
                        && !self.isHyphenOrKhinKey(char) && !self.isIllegal()) {
                        _ = self.commitCurrent();
                        self.candidateViewModel.reset()
                    }
                    
                    if (modifiers.contains(.shift) || modifiers.contains(.capsLock)) {
                        // shif xor caplocks
                        char = char.uppercased();
                    }
                } else if (self.isClassicMode()) {
                    if (modifiers.contains(.shift) || modifiers.contains(.capsLock)) {
                        // shif xor caplocks
                        char = char.uppercased();
                    }

                    // check previous char is punctuation
                    let punctuations = ".,!?()'\":<>;+=_[]「」‘’『』々〱〈《<«〉》>»+＋⁺+⁺=·＝〓_—＿⁻_—⁻〔【〖〕】〗"
                    let text = self.currentDisplayText()
                    // A second hyphen/khin key must reach the engine so it can
                    // cancel the punctuation it just inserted (d+d -> "d",
                    // v+v -> "v") and release the buffer for foreign typing.
                    // Committing here would swallow it instead.
                    if (text.count > 0 && punctuations.contains(text.last!)
                        && !self.isHyphenOrKhinKey(char)) {
                        _ = self.commitCurrent();
                        self.candidateViewModel.reset()
                    }
                }
                self.candidateViewModel.handleChar(char)
                if (self.isCommited()) {
                    client.insert(self.currentDisplayText())
                    self.reset()
                } else {
                    self.resetWindow()
                    client.mark(self.currentDisplayText())
                }
                return true
            case .number(let num):
                if (modifiers.contains(.shift) && self.isClassicMode()) {
                    if (num == 1) {
                        return self.handlePunctuation("!");
                    } else if (num == 9) {
                        return self.handlePunctuation("(");
                    } else if (num == 0) {
                        return self.handlePunctuation(")");
                    }
                }

                if (modifiers.contains(.shift) || modifiers.contains(.capsLock)) {
                    _ = self.commitAll();
                    self.candidateViewModel.reset()
                    return false;
                }
                log.debug("handle number " + String(num))
                self.candidateViewModel.handleChar(String(num))
                if (self.isManualMode()) {
                    if (self.isCommited()) {
                        client.insert(self.currentDisplayText())
                        self.reset()
                    } else {
                        self.resetWindow()
                        client.mark(self.currentDisplayText())
                    }
                } else if (self.isClassicMode()) {
                    if (self.isCommited()) {
                        log.debug("handle number for commit ")
                        client.insert(self.getCommitedText());
                    }
                    self.resetWindow()
                    client.mark(self.currentDisplayText())
                } else {
                    self.resetWindow()
                }
                return true
            case .punctuation(let ch):
                log.debug("handle punctuation " + ch)
                if (self.isClassicMode()) {
                    if (".,'=[];".contains(ch) && !modifiers.contains(.shift)) {
                        return self.handlePunctuation(ch);
                    } else if (ch == "/" && modifiers.contains(.shift)) {
                        return self.handlePunctuation("?");
                    } else if (ch == "'" && modifiers.contains(.shift)) {
                        return self.handlePunctuation("\"");
                    } else if (ch == "," && modifiers.contains(.shift)) {
                        return self.handlePunctuation("<");
                    } else if (ch == "." && modifiers.contains(.shift)) {
                        return self.handlePunctuation(">");
                    } else if (ch == "=" && modifiers.contains(.shift)) {
                        return self.handlePunctuation("+");
                    } else if (ch == "-" && modifiers.contains(.shift)) {
                        return self.handlePunctuation("_");
                    } else if (ch == "`" && modifiers.contains(.shift)) {
                        // Shift + backquote must reach the engine, which
                        // direct-outputs "~" as "〜" in Hanji-first, like
                        // Windows does. Falling through would let the system
                        // insert a plain ASCII tilde instead.
                        return self.handlePunctuation("~");
                    }
                }
            default:
                log.debug("key is special key")
        }

        if (!self.isEdited()) {
            // if key is space, and classic mode, and hanji first, then don't reset window
            if (self.isClassicMode() && self.isHanjiFirst() && event.keyCode.representative == .space && modifiers.contains(.shift)) {
                client.insert("　")
                return true
            }
            return false
        }
        
        if (self.isManualMode()) {
            switch event.keyCode.representative {
                case .enter:
                    fallthrough
                case .space:
                    fallthrough
                case .punctuation:
                    fallthrough
                case .arrow:
                    fallthrough
                case .tab:
                    _ = self.commitAll();
                    self.candidateViewModel.reset()
                    return false;
                case .backspace:
                    self.candidateViewModel.handleBackspace()
                case .escape:
                    self.reset()
                    client.clearMarkedText()
                    return true
                default:
                    log.debug("default handled")
                    _ = self.commitAll();
                    self.candidateViewModel.reset()
                    return false
            }
        } else {
            switch event.keyCode.representative {
                case .enter:
                    self.candidateViewModel.handleEnter()
                case .backspace:
                    self.candidateViewModel.handleBackspace()
                case .escape:
                    self.reset()
                    client.clearMarkedText()
                    return true
                case .space:
                    self.candidateViewModel.handleSpace(modifiers.contains(.shift))
                case .tab:
                    self.candidateViewModel.handleTab(modifiers.contains(.shift))
                case .arrow(Direction.up):
                    self.candidateViewModel.handleArrowUp()
                case .arrow(Direction.down):
                    self.candidateViewModel.handleArrowDown()
                case .arrow(Direction.left):
                    // Classic mode drops out of the composition, like Windows:
                    // the pre-edit is released as-is and the key is swallowed.
                    if (self.isClassicMode()) {
                        return self.releaseComposition()
                    }
                    _ = self.commitAll()
                    self.candidateViewModel.reset()
                    return false
                case .arrow(Direction.right):
                    // Like Windows: with the candidate menu open the right
                    // arrow acts as enter, otherwise the engine ignores it.
                    if (!self.isCandidateListOpen()) {
                        return true
                    }
                    self.candidateViewModel.handleEnter()
                default:
                    log.debug("default handled")
                    _ = self.commitAll();
                    self.candidateViewModel.reset()
                    return false
            }
        }
        if (self.isClassicMode() && self.isCommited()) {
            client.insert(self.getCommitedText());
            self.resetWindow()
            client.mark(self.currentDisplayText())
        } else if (self.isEdited()) {
            self.resetWindow()
            client.mark(self.currentDisplayText())
        } else {
            self.reset()
            client.clearMarkedText()
        }
        return true 
    }
}
