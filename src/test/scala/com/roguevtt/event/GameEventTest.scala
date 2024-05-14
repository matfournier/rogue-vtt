package com.roguevtt.event

import com.roguevtt.event.GameEvent.*
import com.roguevtt.game.Entity
import com.roguevtt.game.EntityType.PLAYER
import zio.json.*

import java.nio.charset.StandardCharsets

class GameEventTest extends munit.FunSuite {

  test("serialization") {
    val addToken = AddToken(Entity(PLAYER, 10, 15, "C", "some-id", "Charlie"))
    val fill = Fill(Bounds(0, 10, 0, 15), 0, 20)

    val addTokenEvent = GameMessage(addToken, "game-id", Some("player"), Some("level-id"))
    val fillTokenEvent = GameMessage(fill, "game-id", Some("player"), Some("level-id"))

    println(addTokenEvent.toJson)
    println(fillTokenEvent.toJson)

    println(fillTokenEvent.toJson.getBytes(StandardCharsets.UTF_8).length)
    assert(9 == 9)


  }

}
