fn main() {
    MainWindow::new().run();
}

slint::slint! {
    struct TileData := {
        image: image,
        image_visible: bool,
        solved: bool,
    }

    MemoryTile := Rectangle {
        callback clicked;
        property <bool> open_curtain;
        property <bool> solved;
        property <image> icon;

        width: 74px;
        height: 74px;
        background:solved ? #1f830bd8 : #d1d6e4;
        animate background {duration: 800ms;}
    
        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
        }

        //Left curtain
        Rectangle { 
            background: #2655ac;
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in;}
         }

         //Right curtain
        Rectangle {
            background: #2655ac;
            x: open_curtain ? parent.width : (parent.width / 2);
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width {duration: 250ms; easing: ease-in;}
            animate x {duration: 250ms; easing: ease-in;}
        }

        TouchArea { 
            clicked => {
                // Deligate to the user of this element
                root.clicked();
            }
         }
    }
MainWindow := Window {
    width: 350px;
    height: 350px;

    property <[TileData]> memory_tiles: [
        { image: @image-url("icons/Leeds_United.png")},
        { image: @image-url("icons/arsenal.png")},
        { image: @image-url("icons/Chelsea.png")},
        { image: @image-url("icons/southampton.png")},
        { image: @image-url("icons/manchester-city.png")},
        { image: @image-url("icons/Manchester_United.png")},
        { image: @image-url("icons/Fulham.png")},
        { image: @image-url("icons/bournemouth.png")},
        { image: @image-url("icons/Brighton.png")},
        { image: @image-url("icons/villa.png")},
        { image: @image-url("icons/everton.png")},
        { image: @image-url("icons/Leicester.png")},
        { image: @image-url("icons/liverpool.png")},
        { image: @image-url("icons/newcastle.png")},
        { image: @image-url("icons/palace.png")},
        { image: @image-url("icons/west_ham.png")},
        { image: @image-url("icons/spurs.png")},
        { image: @image-url("icons/wolves.png)")},
    ];
    for tile[i] in memory_tiles : MemoryTile {
        x: mod(i, 4) *80px;
        y: floor(i/4) *80px;
        width: 74px;
        height: 74px;
        icon: tile.image;
        open_curtain: tile.image_visible || tile.solved;
        // propagate the solved status from the model to the tile
        solved: tile.solved;
        clicked => {
            tile.image_visible = !tile.image_visible;
        }
    }
}
}