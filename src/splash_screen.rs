use std::time::Duration;

use bevy::prelude::*;
use std::path::Path;
use bevy::window::WindowDescriptor;
use bevy::render::render_resource::{TextureDimension, TextureFormat, TextureUsages};
use bevy::utils::HashMap;

use crate::{game_settings::LogicalGameState, components::{is_loaded, AssetHandler}};

use ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, Pixel};
use ffmpeg::frame::Video;
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};


#[derive(Default)]
struct VideoResource {
    video_players: HashMap<Entity, VideoPlayerNonSendData>,
}

struct VideoPlayerNonSendData {
    decoder: ffmpeg::decoder::Video,
    input_context: ffmpeg::format::context::Input,
    scaler_context: Context,
}

impl VideoPlayer {
    fn new<'a, P>(
        path: P,
        mut images: ResMut<Assets<Image>>,
    ) -> Result<(VideoPlayer, VideoPlayerNonSendData), ffmpeg::Error>
    where
        P: AsRef<Path>,
    {
        let input_context = input(&path)?;

        // initialize decoder
        let input_stream = input_context
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input_stream.index();

        let context_decoder =
            ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
        let decoder = context_decoder.decoder().video()?;

        // initialize scaler
        let scaler_context = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGBA,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        // create image texture
        let mut image = Image::new_fill(
            bevy::render::render_resource::Extent3d {
                width: decoder.width(),
                height: decoder.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &Color::PINK.as_rgba_u32().to_le_bytes(),
            TextureFormat::Rgba8UnormSrgb,
        );
        image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;

        let image_handle = images.add(image);

        Ok((
            VideoPlayer {
                image_handle,
                video_stream_index,
            },
            VideoPlayerNonSendData {
                decoder,
                input_context,
                scaler_context,
            },
        ))
    }
}

#[derive(Component)] 
struct VideoPlayer {
    image_handle: Handle<Image>,
    video_stream_index: usize,
}

pub struct SplashScreen;

const SPLASH_IMAGE: &'static str = "branding/matoface_logo.png";


#[derive(Resource)]
struct LocalTimer {
    timer: Timer,
}

#[derive(Component)]
struct SplashComponent; // so we can clean up later (destroy it all >:) ) 

/* BUILD */
impl Plugin for SplashScreen {
    fn build(&self, app: &mut App) {
        /* TODO: Splash screen stuff :> */
        app
            .init_non_send_resource::<VideoResource>()
            .add_startup_system(initialize_ffmpeg)
            .add_system_set(SystemSet::on_enter(LogicalGameState::Splash).with_system(spawn_spash_screen))
            .add_system_set(SystemSet::on_update(LogicalGameState::Splash)
                .with_system(play_video)
            )
            .insert_resource( LocalTimer {
                timer : Timer::new(Duration::from_secs(5), TimerMode::Once), 
            });
    }
}

#[derive(Component)]
struct LoadingAsset;

fn initialize_ffmpeg() {
    ffmpeg::init().unwrap();
}

fn spawn_spash_screen(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    mut video_resource: NonSendMut<VideoResource>,
) {
    let (video_player, video_player_non_send) = VideoPlayer::new("./assets/branding/matos_intro.mp4", images).unwrap();

    commands
    .spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        let entity = parent
            .spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(1024.0), Val::Px(672.0)),
                    ..default()
                },
                image: video_player.image_handle.clone().into(),
                ..default()
            })
            .insert(video_player)
            .insert(LoadingAsset)
            .id();
        video_resource
            .video_players
            .insert(entity, video_player_non_send);
    }).insert(LoadingAsset);
}


fn play_video(
    mut commands: Commands,
    mut video_player_query: Query<(&mut VideoPlayer, Entity)>,
    mut video_resource: NonSendMut<VideoResource>,
    mut images: ResMut<Assets<Image>>,
    mut game_state: ResMut<State<LogicalGameState>>,
    time: Res<Time>,
    splash_entities: Query<Entity, With<LoadingAsset>>
) {
    //if time.delta_seconds() < 1./20. { return; }
    for (video_player, entity) in video_player_query.iter_mut() {
        let video_player_non_send = video_resource.video_players.get_mut(&entity).unwrap();
        // read packets from stream until complete frame received
        while let Some((stream, packet)) = video_player_non_send.input_context.packets().next() {
            // check if packets is for the selected video stream
            if stream.index() == video_player.video_stream_index {
                // pass packet to decoder
                video_player_non_send.decoder.send_packet(&packet).unwrap();
                let mut decoded = Video::empty();
                // check if complete frame was received
                if let Ok(()) = video_player_non_send.decoder.receive_frame(&mut decoded) {
                    let mut rgb_frame = Video::empty();
                    // run frame through scaler for color space conversion
                    video_player_non_send
                        .scaler_context
                        .run(&decoded, &mut rgb_frame)
                        .unwrap();
                    // update data of image texture
                    let image = images.get_mut(&video_player.image_handle).unwrap();
                    image.data.copy_from_slice(rgb_frame.data(0));
                    return;
                }
            }
        }
        // no frame received
        // signal end of playback to decoder
        match video_player_non_send.decoder.send_eof() {
            Err(ffmpeg::Error::Eof) => {
                game_state.set(LogicalGameState::Menu);
                for splash_ent in splash_entities.iter() {
                    commands.entity(splash_ent).despawn();
                }
            }
            other => other.unwrap(),
        }
    }
}