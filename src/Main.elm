module Main exposing (..)

import Browser
import Html exposing (Html, button, div, h1, h2, input, li, small, text, ul)
import Html.Attributes exposing (value)
import Html.Events exposing (onClick, onInput)
import Set exposing (Set)



-- MAIN


main =
    Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
    { currentTodo : String
    , todos : Set String
    }


init : Model
init =
    { currentTodo = ""
    , todos = Set.empty
    }



-- UPDATE


type Msg
    = MessageChanged String
    | AddTodo


update : Msg -> Model -> Model
update msg model =
    case msg of
        MessageChanged t ->
            { model | currentTodo = t }

        AddTodo ->
            { model | currentTodo = "", todos = Set.insert model.currentTodo model.todos }



-- VIEW


view : Model -> Html Msg
view model =
    div []
        ([ h1 [] [ text "Simple Todo App" ]
         , input [ onInput MessageChanged, value model.currentTodo ] []
         , button [ onClick AddTodo ] [ text "Add todo" ]
         ]
            ++ viewTodos model
        )


viewTodos : Model -> List (Html Msg)
viewTodos model =
    [ h2 [] [ text "Todo" ]
    , if Set.size model.todos > 0 then
        ul []
            (List.map (\todo -> li [] [ text todo, text " " ]) (Set.toList model.todos))

      else
        small [] [ text "Hooray! Everything's done :-)" ]
    ]
