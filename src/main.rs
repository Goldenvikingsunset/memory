fn main() {
    use slint::Model;

    let main_window = MainWindow::new();

    //Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // Duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    // Randomly mix the tiles
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the suffled Vec to the model prperty
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.into());

    main_window.run();
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
        background:solved ? #1f830bd8 : #fdfdff;
        animate background {duration: 800ms;}
    
        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
        }

        //Left curtain
        Rectangle { 
            background: #510a94;
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in;}
         }

         //Right curtain
        Rectangle {
            background: #510a94;
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
    width: 395px;
    height: 640px;
    background: #04795f;

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
        { image: @image-url("icons/wolves.png")},
        { image: @image-url("icons/brentford.png")},
        { image: @image-url("icons/forest.png")},
    ];
    for tile[i] in memory_tiles : MemoryTile {
        x: mod(i, 5) *80px;
        y: floor(i/5) *80px;
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