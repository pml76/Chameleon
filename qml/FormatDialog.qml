import QtQuick
import QtQuick.Controls.FluentWinUI3

import chameleon.dialogs.format

Dialog {

    x: 100
    y: 100
    width: 800
    height: 400

    signal accepted
    signal rejected

    title: "Title"
    standardButtons: Dialog.Ok | Dialog.Cancel
    popupType: Popup.Window

    modal: true
    dim: true

    LocaleSelectorModel {
        id: localeSelectorModel
    }

    ComboBox {
        id: myComboBox

        currentIndex: localeSelectorModel.getCurrentIndex()

        textRole: "text"
        valueRole: "value"
        model: localeSelectorModel
    }
}
