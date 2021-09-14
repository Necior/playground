module Main exposing (main)

import Browser
import Html exposing (Html, button, div, h1, small, span, strong, text)
import Html.Attributes exposing (style)
import Html.Events exposing (onClick)
import Time


main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }


init : () -> ( Model, Cmd Msg )
init _ =
    let
        minutes =
            \s -> 60 * s

        work =
            { step = Work, duration = 25 |> minutes }

        break =
            { step = Break, duration = 5 |> minutes }

        longBreak =
            { step = LongBreak, duration = 30 |> minutes }
    in
    ( { pomodoro = [ work, break, work, break, work, break, work, longBreak ]
      , currentIndex = 0
      , elapsedSeconds = 0
      , state = Reseted
      }
    , Cmd.none
    )


subscriptions model =
    case model.state of
        Paused ->
            Sub.none

        Reseted ->
            Sub.none

        Running ->
            Time.every 1000 (SecondsPassed 1)


type alias Model =
    { pomodoro : Pomodoro
    , currentIndex : Int
    , elapsedSeconds : Int
    , state : State
    }


type State
    = Running
    | Paused
    | Reseted


type alias Pomodoro =
    List TimedStep


type Step
    = Work
    | Break
    | LongBreak


type alias TimedStep =
    { step : Step
    , duration : Int
    }


type Msg
    = FinishCurrent
    | Reset
    | SecondsPassed Int Time.Posix
    | Pause
    | Unpause


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    let
        newModel =
            case msg of
                FinishCurrent ->
                    if model.currentIndex + 1 == List.length model.pomodoro then
                        { model | currentIndex = 0 }

                    else
                        { model | currentIndex = model.currentIndex + 1, elapsedSeconds = 0 }

                Reset ->
                    { model | currentIndex = 0, elapsedSeconds = 0, state = Reseted }

                SecondsPassed seconds _ ->
                    { model | elapsedSeconds = model.elapsedSeconds + seconds }

                Pause ->
                    { model | state = Paused }

                Unpause ->
                    { model | state = Running }
    in
    ( newModel, Cmd.none )


view : Model -> Html Msg
view model =
    div []
        ([ showState model.state
         , h1 [] [ text "Control panel" ]
         , if model.state == Running then
            button [ onClick FinishCurrent ] [ text "Proceed" ]

           else
            button [ onClick Reset ] [ text "Reset" ]
         , showStateButton model
         , h1 [] [ text "Progress" ]
         ]
            ++ showPomodoro model
        )


showState : State -> Html Msg
showState state =
    div []
        [ h1 [] [ text "State" ]
        , text (stateLabel state)
        ]


stateLabel : State -> String
stateLabel s =
    case s of
        Running ->
            "Running"

        Paused ->
            "Paused"

        Reseted ->
            "Not started yet"


stepLabel : Step -> String
stepLabel s =
    case s of
        Work ->
            "Work"

        Break ->
            "Break"

        LongBreak ->
            "Long break"


showStateButton : Model -> Html Msg
showStateButton model =
    case model.state of
        Paused ->
            button [ onClick Unpause ] [ text "Unpause" ]

        Reseted ->
            button [ onClick Unpause ] [ text "Start" ]

        Running ->
            button [ onClick Pause ] [ text "Pause" ]


showPomodoro : Model -> List (Html Msg)
showPomodoro model =
    List.indexedMap (showStep model.currentIndex model.elapsedSeconds) model.pomodoro


showStep : Int -> Int -> Int -> TimedStep -> Html Msg
showStep i seconds j p =
    let
        stepText =
            text (stepLabel p.step ++ " (" ++ String.fromInt (p.duration // 60) ++ " min)")

        progressText =
            progress seconds p.duration
    in
    if i == j then
        div
            [ style "color"
                (if seconds <= p.duration then
                    "green"

                 else
                    "red"
                )
            ]
            [ text "→ ", stepText, text (" (" ++ progressText ++ " done)") ]

    else
        div [ style "opacity" "0.6" ] [ small [] [ stepText ] ]


progress : Int -> Int -> String
progress cur max =
    if cur > max then
        "over 100%"

    else
        String.fromInt (100 * cur // max) ++ "%"
