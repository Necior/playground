module Main exposing (..)

import Browser
import Html exposing (Attribute, Html, button, div, h1, h2, input, li, small, text, ul)
import Html.Attributes exposing (autofocus, value)
import Html.Events exposing (keyCode, on, onClick, onInput)
import Json.Decode
import Set exposing (Set)



-- MAIN


main =
    Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
    { currentTodo : String
    , todos : List String
    }


init : Model
init =
    { currentTodo = ""
    , todos = []
    }



-- UPDATE


type Msg
    = MessageChanged String
    | AddTodo
    | MarkAsDone String


update : Msg -> Model -> Model
update msg model =
    case msg of
        MessageChanged t ->
            { model | currentTodo = t }

        AddTodo ->
            { model | currentTodo = "", todos = model.currentTodo :: model.todos }

        MarkAsDone t ->
            { model | todos = List.filter (\task -> task /= t) model.todos }



-- VIEW


view : Model -> Html Msg
view model =
    div []
        ([ h1 [] [ text "Simple Todo App" ]
         , input [ autofocus True, onInput MessageChanged, value model.currentTodo, onEnter AddTodo ] []
         , button [ onClick AddTodo ] [ text "Add todo" ]
         ]
            ++ viewTodos model
        )


viewTodos : Model -> List (Html Msg)
viewTodos model =
    [ h2 [] [ text "Todo" ]
    , if List.length model.todos > 0 then
        ul []
            (List.map (\todo -> li [] [ button [ onClick (MarkAsDone todo) ] [ text "Done" ], text (" " ++ todo) ]) model.todos)

      else
        small [] [ text "Hooray! Everything's done :-)" ]
    ]



-- stolen from BSD-licensed Evan Czaplicki's TodoMVC
-- https://github.com/evancz/elm-todomvc/blob/f236e7e56941c7705aba6e42cb020ff515fe3290/src/Main.elm#L248-L257


onEnter : Msg -> Attribute Msg
onEnter msg =
    let
        isEnter code =
            if code == 13 then
                Json.Decode.succeed msg

            else
                Json.Decode.fail "not Enter"
    in
    on "keydown" (Json.Decode.andThen isEnter keyCode)
