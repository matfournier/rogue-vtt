package com.roguevtt.game

import com.roguevtt.game.EntityType.PLAYER
import com.roguevtt.game.LevelType.DUNGEON

import java.util.UUID
import scala.collection.mutable

import zio.json._


case class Point(x: Int, y: Int)

object Point {
  implicit val encoder: JsonEncoder[Point] = DeriveJsonEncoder.gen[Point]
  implicit val decoder: JsonDecoder[Point] = DeriveJsonDecoder.gen[Point]
}

case class Tile(x: Int, y: Int, idx: Int)

object Tile {
  implicit val encoder: JsonEncoder[Tile] = DeriveJsonEncoder.gen[Tile]
  implicit val decoder: JsonDecoder[Tile] = DeriveJsonDecoder.gen[Tile]
}


// need to encode these as 0, 1 rather than their types
// need to write a custom encoder/decoder "fromOrdinal"
// should be able to contraMap it into 0,1
// TODO figure out how to handle failure inside of that contraMap?
enum LevelType {
  case DUNGEON, OVERLAND
}

object LevelType {
  implicit val encoder: JsonEncoder[LevelType] = DeriveJsonEncoder.gen[LevelType]
  implicit val decoder: JsonDecoder[LevelType] = DeriveJsonDecoder.gen[LevelType]
}

enum Tileset {
  case FLOOR, FEATURE
}

object Tileset {
  implicit val encoder: JsonEncoder[Tileset] = DeriveJsonEncoder.gen[Tileset]
  implicit val decoder: JsonDecoder[Tileset] = DeriveJsonDecoder.gen[Tileset]
}

enum EntityType {
  case PLAYER, NPC
}

object EntityType {
  implicit val encoder: JsonEncoder[EntityType] = DeriveJsonEncoder.gen[EntityType]
  implicit val decoder: JsonDecoder[EntityType] = DeriveJsonDecoder.gen[EntityType]
}

case class Entity(kind: EntityType, x: Int, y: Int, character: String, id: String, description: String) {
  def move(xx: Int, yy: Int): Entity = this.copy(x = xx, y=yy)
}
object Entity {
  implicit val encoder: JsonEncoder[Entity] = DeriveJsonEncoder.gen[Entity]
  implicit val decoder: JsonDecoder[Entity] = DeriveJsonDecoder.gen[Entity]
}

case class Entities(private val players: Map[String, Entity], private val npcs: Map[String, Entity]) {
  def mutable: MutableEntities = MutableEntities.from(players, npcs)
}

object Entities {
  implicit val encoder: JsonEncoder[Entities] = DeriveJsonEncoder.gen[Entities]
  implicit val decoder: JsonDecoder[Entities] = DeriveJsonDecoder.gen[Entities]
}

class MutableEntities(private val players: mutable.HashMap[String, Entity], npcs: mutable.HashMap[String, Entity]) {
  def add(entity: Entity): MutableEntities = {
    which(entity) += entity.id -> entity
    this
  }

  def remove(entity: Entity): Option[Entity] = which(entity).remove(entity.id)

  def move(entity: Entity, x: Int, y: Int): Unit =
    which(entity).remove(entity.id).foreach(e => add(e.move(x, y)))


  private def which(entity: Entity): collection.mutable.Map[String, Entity] = entity.kind match
    case EntityType.PLAYER => players
    case EntityType.NPC => npcs

  def immutable: Entities = {
    Entities(players.toMap, npcs.toMap)
  }
}

object MutableEntities {
  def from(players: Map[String, Entity], npcs: Map[String, Entity]): MutableEntities = {
    new MutableEntities(mutable.HashMap.from(players), mutable.HashMap.from(npcs))
  }
}

case class Level[T](kind: LevelType, description: String, id: String, dimension: Point, tiles: T, features: T) {
  def intToIdx(x: Int, y: Int): Option[Int] =
    if (x > dimension.x || y > dimension.y) None else Some(y * dimension.y + x)

  def pointToIdx(p: Point): Option[Int] = intToIdx(p.x, p.y)

  def idxToPoint(idx: Int): Point =
    Point(idx % dimension.y,idx / dimension.y)
}

case class GameSate[T](level: Level[T], entities: Entities, id: String)

object GameSate {
  // not sure I want this at all
  def make(description: String, dimension: Point): GameSate[mutable.Map[Int, Int]] = {
    val entities = Entities(Map[String, Entity](), Map[String, Entity]())
    val level: Level[mutable.Map[Int, Int]] = Level(kind = DUNGEON, description, UUID.randomUUID().toString, dimension, new mutable.HashMap[Int, Int]().withDefaultValue(0), new mutable.HashMap[Int, Int]())
    GameSate(level, entities, UUID.randomUUID().toString)
  }
}
