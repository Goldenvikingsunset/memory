fn main() {
    MainWindow::new().run();
}

slint::slint! {
    MemoryTile := Rectangle {
        width: 64px;
        height: 64px;
        background: #3960d5;
    
        Image {
            source:@image-url("memory/target/icons/bus.png");
            width: parent.width;
            height: parent.height;
        }
    }
MainWindow := Window {
    MemoryTile {}
}
}